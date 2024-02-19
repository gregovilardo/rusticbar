mod imp;

use glib::{clone, Object};
use gtk::{gio, glib};

glib::wrapper! {
    pub struct InternetWidget(ObjectSubclass<imp::InternetWidget>)
    @extends gtk::Box, gtk::Widget,
    @implements gtk::Accessible, gtk::Buildable, gtk::ConstraintTarget, gtk::Orientable;
}

impl Default for InternetWidget {
    fn default() -> Self {
        Self::new()
    }
}

impl InternetWidget {
    pub fn new() -> Self {
        Object::builder().build()
    }
}
