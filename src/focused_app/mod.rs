mod imp;
use glib::{clone, Object};
use gtk::{gio, glib, subclass::prelude::ObjectSubclassIsExt};
use swayipc::{self, WindowEvent};

glib::wrapper! {
    pub struct FocusedAppWidget(ObjectSubclass<imp::FocusedAppWidget>)
    @extends gtk::Box, gtk::Widget,
    @implements gtk::Accessible, gtk::Buildable, gtk::ConstraintTarget, gtk::Orientable;
}

impl Default for FocusedAppWidget {
    fn default() -> Self {
        Self::new()
    }
}

impl FocusedAppWidget {
    pub fn new() -> Self {
        Object::builder().build()
    }
    //TODO:How to get the abbreviation
    pub fn focused_app_from_sway_ev(&self, ev: std::boxed::Box<WindowEvent>) {
        if ev.change == swayipc::WindowChange::Close {
            self.imp().focused_app_label.set_text("");
        }
        if ev.change == swayipc::WindowChange::Focus || ev.change == swayipc::WindowChange::Title {
            // Also Title event
            if ev.container.focused {
                if let Some(app_name) = ev.container.name {
                    self.imp().focused_app_label.set_text(&app_name);
                }
            }
        }
    }
    pub fn setup_focused(&self, name: &str) {
        self.imp().focused_app_label.set_text(&name);
    }
}
