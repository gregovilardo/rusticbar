mod imp;
use glib::{clone, Object};
use gtk::{gio, glib, subclass::prelude::ObjectSubclassIsExt};
use std::process::Command;
use swayipc::{self, WindowEvent};

glib::wrapper! {
    pub struct GammarelayWidget(ObjectSubclass<imp::GammarelayWidget>)
    @extends gtk::Box, gtk::Widget,
    @implements gtk::Accessible, gtk::Buildable, gtk::ConstraintTarget, gtk::Orientable;
}

impl Default for GammarelayWidget {
    fn default() -> Self {
        Self::new()
    }
}

impl GammarelayWidget {
    pub fn new() -> Self {
        Object::builder().build()
    }
    fn run_gammarelay(&self) {
        self.imp().gammarelay_label.set_text("10");
        let mut gammarelay = Command::new("wl-gammarelay-rs");
        gammarelay.arg("run");
        let mut child = gammarelay.spawn().expect("failed to execute process");
        println!("{:?}", child);
        let output = Command::new("busctl")
            .arg("--user")
            .arg("set-property")
            .arg("rs.wl-gammarelay")
            .arg("/")
            .arg("rs.wl.gammarelay")
            .arg("Temperature")
            .arg("q")
            .arg("3000")
            .output()
            .expect("Failed to execute command");

        // Print command output
        println!("Command output: {:?}", output);
        std::thread::sleep(std::time::Duration::new(3, 0));
        let output = Command::new("busctl")
            .arg("--user")
            .arg("set-property")
            .arg("rs.wl-gammarelay")
            .arg("/")
            .arg("rs.wl.gammarelay")
            .arg("Temperature")
            .arg("q")
            .arg("5000")
            .output()
            .expect("Failed to execute command");

        // Print command output
        println!("Command output: {:?}", output);
    }
}
