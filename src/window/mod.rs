mod imp;

use std::cell::Cell;
use std::rc::Rc;

use chrono::Local;
use glib::{clone, Object};
use gtk::gdk::{Display, Monitor};
use gtk::glib::ControlFlow;
use gtk::prelude::DisplayExt;
use gtk::subclass::prelude::ObjectSubclassIsExt;
use gtk::{gio, glib, Application, ListItem, NoSelection, SignalListItemFactory};
use gtk::{prelude::*, Label};
use swayipc::WorkspaceChange;

use crate::ws_object::{self, WsObject};
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
        // let height = monitor.height() / 120;
        let height = 26;
        let layout_manager = self.layout_manager();
        println!("{:?}", layout_manager);
        let childs = self.child();
        println!("{:?}", childs);
        self.set_has_tooltip(false);
        self.set_size_request(width, height);
        self.set_default_size(width, height);
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
        self.imp().wss_column.set_model(Some(&selection_model));
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
        self.imp().wss_column.set_factory(Some(&factory));
    }

    fn setup_sway_events(&self) {
        let conn = swayipc::Connection::new().expect("conn");
        let (sender, receiver) = async_channel::unbounded();

        gio::spawn_blocking(move || {
            let ws_event = conn
                .subscribe([swayipc::EventType::Workspace])
                .expect("ws event");

            for e in ws_event {
                if let swayipc::Event::Workspace(ev) = e.expect("event") {
                    let _s = sender.send_blocking(ev);
                }
            }
        });

        glib::spawn_future_local(clone! (@weak self as window => async move {
            while let Ok(ev) = receiver.recv().await {
                match ev.change {
                    swayipc::WorkspaceChange::Init => {
                        let cn = ev.current.expect("event");
                        let ws_object = WsObject::new(
                            cn.num.expect("num") as u8,
                            cn.name.expect("name"),
                            cn.focused,
                        );
                        window.new_workspace(ws_object);
                    },
                    swayipc::WorkspaceChange::Focus => {
                    },
                    swayipc::WorkspaceChange::Empty => {
                        if let Some(ce)  = ev.current {
                            window.delete_workspace(ce.num.expect("ws number") as u8);
                            println!("current {}", ce.name.expect("name"));
                        }
                        if let Some(ce)  = ev.old {
                            println!("old {}", ce.name.expect("name"));
                        }

                        // let cn = ev.current.expect("event");
                        // let ws_object = WsObject::new(
                        //     cn.num.expect("num") as u8,
                        //     cn.name.expect("name"),
                        //     cn.focused,
                        // );
                        // window.new_workspace(ws_object);
                    },
                    _ => {},
                }
                // println!("{}", cn.name.expect("name on await"));
            }
        }));
    }
}

pub fn get_time() -> String {
    format!("{}", Local::now().format("%Y-%m-%d %H:%M:%S"))
}

// let mut conn = swayipc::Connection::new().expect("conn");
// // let workspaces = conn.get_workspaces().expect("workspaces");
// // for w in workspaces {
// //     println!("{:?}", w);
// // }
//
