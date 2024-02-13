mod imp;

use glib::Object;
use gtk::prelude::*;
use gtk::subclass::prelude::*;
use gtk::{glib, pango};
use pango::{AttrInt, AttrList};

glib::wrapper! {
    pub struct VolWidget(ObjectSubclass<imp::VolWidget>)
    @extends gtk::Box, gtk::Widget,
    @implements gtk::Accessible, gtk::Buildable, gtk::ConstraintTarget, gtk::Orientable;
}

impl Default for VolWidget {
    fn default() -> Self {
        Self::new()
    }
}

impl VolWidget {
    pub fn new() -> Self {
        Object::builder().build()
    }
    fn setup_volume_event(&self) {}
}
