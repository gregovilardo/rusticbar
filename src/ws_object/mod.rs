mod imp;

use glib::Object;
use gtk::glib;

glib::wrapper! {
    pub struct WsObject(ObjectSubclass<imp::WsObject>);
}

impl WsObject {
    pub fn new(num: u32, name: String, focus: bool, empty: bool) -> Self {
        Object::builder()
            .property("num", num)
            .property("name", name)
            .property("focus", focus)
            .property("empty", empty)
            .build()
    }
}

#[derive(Default)]
pub struct WsData {
    pub num: u32,
    pub name: String,
    pub focus: bool,
    pub empty: bool,
}
