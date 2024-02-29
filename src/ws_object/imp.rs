use std::cell::RefCell;

use glib::Properties;
use gtk::glib;
use gtk::prelude::*;
use gtk::subclass::prelude::*;

use super::WsData;

// Object holding the state
#[derive(Properties, Default)]
#[properties(wrapper_type = super::WsObject)]
pub struct WsObject {
    #[property(name = "num", get, set, type = i32, member = num)]
    #[property(name = "name", get, set, type = String, member = name)]
    #[property(name = "focused", get, set, type = bool, member = focused)]
    pub data: RefCell<WsData>,
}

// The central trait for subclassing a GObject
#[glib::object_subclass]
impl ObjectSubclass for WsObject {
    const NAME: &'static str = "WsObject";
    type Type = super::WsObject;
}

// Trait shared by all GObjects
#[glib::derived_properties]
impl ObjectImpl for WsObject {}
