mod imp;
use chrono::Local;
use glib::{clone, Object};
use gtk::{gio, glib, subclass::prelude::ObjectSubclassIsExt};

glib::wrapper! {
    pub struct TimeWidget(ObjectSubclass<imp::TimeWidget>)
    @extends gtk::Box, gtk::Widget,
    @implements gtk::Accessible, gtk::Buildable, gtk::ConstraintTarget, gtk::Orientable;
}

impl Default for TimeWidget {
    fn default() -> Self {
        Self::new()
    }
}

impl TimeWidget {
    pub fn new() -> Self {
        Object::builder().build()
    }
    fn setup_time(&self) {
        let time_label = self.imp().time_label.clone();
        let tick_time = move || {
            let time = get_time();
            time_label.set_text(&time);
            // we could return glib::ControlFlow::Break to stop our clock after this tick
            glib::ControlFlow::Continue
        };

        // executes the closure once every second
        glib::timeout_add_seconds_local(1, tick_time);
    }
}

pub fn get_time() -> String {
    format!("{}", Local::now().format("%Y-%m-%d %H:%M:%S"))
}
