mod imp;

use glib::Object;
use gtk::prelude::*;
use gtk::subclass::prelude::*;
use gtk::{glib, pango};
use pango::{AttrInt, AttrList};

use crate::ws_object::WsObject;

glib::wrapper! {
    pub struct WsWidget(ObjectSubclass<imp::WsWidget>)
    @extends gtk::Box, gtk::Widget,
    @implements gtk::Accessible, gtk::Buildable, gtk::ConstraintTarget, gtk::Orientable;
}

impl Default for WsWidget {
    fn default() -> Self {
        Self::new()
    }
}

impl WsWidget {
    pub fn new() -> Self {
        Object::builder().build()
    }
}
