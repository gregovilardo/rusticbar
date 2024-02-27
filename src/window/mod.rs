mod imp;

use chrono::Local;
use glib::{clone, Object};
use gtk::gdk::{Display, Monitor};
use gtk::prelude::DisplayExt;
use gtk::subclass::prelude::ObjectSubclassIsExt;
use gtk::{gio, glib, Application, Box, ListItem, NoSelection, SignalListItemFactory};
use gtk::{prelude::*, Label};

use crate::systeminfo::SystemInfoWidget;
use crate::volume::VolWidget;
use crate::ws_object::WsObject;
use crate::ws_widget::WsWidget;

glib::wrapper! {
    pub struct Window(ObjectSubclass<imp::Window>)
        @extends gtk::ApplicationWindow, gtk::Window, gtk::Widget,
        @implements gio::ActionGroup, gio::ActionMap, gtk::Accessible, gtk::Buildable,
                    gtk::ConstraintTarget, gtk::Native, gtk::Root, gtk::ShortcutManager;
}

impl Window {
    pub fn new(app: &Application) -> Self {
        // Create new window
        Object::builder().property("application", app).build()
    }

    fn setup_size(&self) {
        let display = Display::default().expect("failed display");
        let monitors = display.monitors();
        let monitor = monitors
            .iter::<Monitor>()
            .last()
            .expect("there is no monitor")
            .expect("failed monitor")
            .geometry();
        let width = monitor.width();
        let height = monitor.height() / 41;
        // let height = 26;
        self.set_size_request(width, height);
        self.set_default_size(width, height);
    }

    fn setup_child_widgets(&self) {
        let gtk_box = self
            .first_child()
            .expect("box")
            .downcast::<Box>()
            .expect("gtk_box");
        gtk_box.append(&self.imp().focused_app_widget);
        gtk_box.append(&self.imp().network_widget);
        gtk_box.append(&VolWidget::new());
        gtk_box.append(&SystemInfoWidget::new());
        gtk_box.append(&self.imp().keyboard_layout_widget);
        gtk_box.append(&self.imp().gammarelay_widget);
    }

    fn workspaces(&self) -> gio::ListStore {
        self.imp()
            .workspaces
            .borrow_mut()
            .clone()
            .expect("could not get current workspaces")
    }

    fn new_workspace(&self, ws_object: WsObject) {
        let workspaces = self.workspaces();
        let mut position = 0;
        while let Some(item) = workspaces.item(position) {
            if item
                .downcast::<WsObject>()
                .expect("has to be ws object")
                .num()
                > ws_object.num()
            {
                self.workspaces().insert(position, &ws_object);
                return;
            }
            position += 1;
        }
        self.workspaces().insert(position, &ws_object);
    }

    fn delete_workspace(&self, num: u8) {
        let workspaces = self.workspaces();
        let mut position = 0;
        while let Some(item) = workspaces.item(position) {
            if item
                .downcast::<WsObject>()
                .expect("has to be ws object")
                .num()
                == num
            {
                workspaces.remove(position);
                break;
            }
            position += 1;
        }
    }

    fn focus_workspace(&self, num: u8, focus_status: bool) {
        let workspaces = self.workspaces();
        let mut position = 0;
        while let Some(item) = workspaces.item(position) {
            let ws_object = item.downcast::<WsObject>().expect("has to be WsObject");
            if ws_object.num() == num {
                ws_object.set_focused(focus_status);
                break;
            }
            position += 1;
        }
    }

    fn setup_workspaces(&self) {
        // Create new model
        let model = gio::ListStore::new::<WsObject>();

        let mut conn = swayipc::Connection::new().expect("conn");
        let ws_vec = conn.get_workspaces().expect("workspaces");

        for w in ws_vec {
            let ws = WsObject::new(w.num.try_into().unwrap(), w.name.clone(), w.focused);
            model.append(&ws);
        }

        self.imp().workspaces.replace(Some(model));

        let selection_model = NoSelection::new(Some(self.workspaces()));
        self.imp().wss_list.set_model(Some(&selection_model));
    }

    fn setup_factory(&self) {
        // Create a new factory
        let factory = SignalListItemFactory::new();

        // Create an empty `WsWidget` during setup
        factory.connect_setup(move |_, list_item| {
            // Create `WsWidget`
            let ws_widget = WsWidget::new();
            list_item
                .downcast_ref::<ListItem>()
                .expect("Needs to be ListItem")
                .set_child(Some(&ws_widget));
        });

        // Tell factory how to bind `WsWidget` to a `WsObject`
        factory.connect_bind(move |_, list_item| {
            // Get `WsObject` from `ListItem`
            let ws_object = list_item
                .downcast_ref::<ListItem>()
                .expect("Needs to be ListItem")
                .item()
                .and_downcast::<WsObject>()
                .expect("The item has to be an `WsObject`.");

            // Get `TaskRow` from `ListItem`
            let ws_widget = list_item
                .downcast_ref::<ListItem>()
                .expect("Needs to be ListItem")
                .child()
                .and_downcast::<WsWidget>()
                .expect("The child has to be a `WsWidget`.");

            ws_widget.bind(&ws_object);
        });

        // Tell factory how to unbind `WsWidget` from `WsObject`
        factory.connect_unbind(move |_, list_item| {
            let ws_widget = list_item
                .downcast_ref::<ListItem>()
                .expect("Needs to be ListItem")
                .child()
                .and_downcast::<WsWidget>()
                .expect("The child has to be a `WsWidget`.");

            ws_widget.unbind();
        });

        // Set the factory of the list view
        self.imp().wss_list.set_factory(Some(&factory));
    }

    fn setup_sway_events(&self) {
        //TODO: Deberia separar esto... generar nuevos componentes y mandarles los eventos
        let mut conn = swayipc::Connection::new().expect("conn");
        let (sender, receiver) = async_channel::unbounded();

        self.imp()
            .keyboard_layout_widget
            .setup_layout(conn.get_inputs());

        gio::spawn_blocking(move || {
            let ws_event = conn
                .subscribe([
                    swayipc::EventType::Workspace,
                    swayipc::EventType::Window,
                    swayipc::EventType::Input,
                ])
                .expect("ws event");

            for e in ws_event {
                let _s = sender.send_blocking(e);
                // if let swayipc::Event::Workspace(ev) = e.expect("event") {
                //     let _s = sender.send_blocking(ev);
                // }
                // if let swayipc::Event::Window(ev) = e.expect("event") {
                //     let _s = sender.send_blocking(ev);
                // }
            }
        });

        glib::spawn_future_local(clone! (@weak self as window => async move {
            while let Ok(ev) = receiver.recv().await {
                        // println!("{:#?}", ev);
                match ev {
                    Ok(swayipc::Event::Workspace(ev)) => {
                        match ev.change {
                            swayipc::WorkspaceChange::Init => {
                                if let Some(current_node)  = ev.current {
                                    let ws_object = WsObject::new(
                                        current_node.num.expect("num") as u8,
                                        current_node.name.expect("name"),
                                        current_node.focused,
                                    );
                                    window.new_workspace(ws_object);
                                    window.imp().focused_app_widget.setup_focused("");
                                }
                            },
                            swayipc::WorkspaceChange::Focus => {
                                if let Some(current_node)  = ev.current {
                                    // window.delete_workspace(current_node.num.expect("ws number") as u8);
                                    window.focus_workspace(current_node.num.expect("num") as u8, true);
                                }
                                if let Some(old_node)  = ev.old {
                                    window.focus_workspace(old_node.num.expect("num") as u8, false);
                                }
                            },
                            swayipc::WorkspaceChange::Empty => {
                                if let Some(current_node)  = ev.current {
                                    window.delete_workspace(current_node.num.expect("ws number") as u8);
                                }
                            },
                            _ => {},
                        }
                    },
                    Ok(swayipc::Event::Window(ev)) => {
                        window.imp().focused_app_widget.focused_app_from_sway_ev(ev);
                    },
                    Ok(swayipc::Event::Input(ev)) => {
                        window.imp().keyboard_layout_widget.layout_from_sway_ev(ev);
                    },
                    _ => {},
                }

            }
        }));
    }
}

pub fn get_time() -> String {
    format!("{}", Local::now().format("%Y-%m-%d %H:%M:%S"))
}
