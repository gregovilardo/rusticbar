mod imp;

use std::cell::Cell;
use std::rc::Rc;

use chrono::Local;
use glib::Object;
use gtk::gdk::{Display, Monitor};
use gtk::glib::ControlFlow;
use gtk::prelude::DisplayExt;
use gtk::subclass::prelude::ObjectSubclassIsExt;
use gtk::{gio, glib, Application};
use gtk::{prelude::*, Label};

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
}

pub fn get_time() -> String {
    format!("{}", Local::now().format("%Y-%m-%d %H:%M:%S"))
}
