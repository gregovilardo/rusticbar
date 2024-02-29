mod imp;

use glib::Object;
use gtk::glib;

glib::wrapper! {
    pub struct WsObject(ObjectSubclass<imp::WsObject>);
}

impl Default for WsObject {
    fn default() -> Self {
        Self::new(0, "None".to_string(), false)
    }
}

impl WsObject {
    pub fn new(num: i32, name: String, focused: bool) -> Self {
        Object::builder()
            .property("num", num)
            .property("name", name)
            .property("focused", focused)
            .build()
    }
}

#[derive(Default)]
pub struct WsData {
    pub num: i32,
    pub name: String,
    pub focused: bool,
}
