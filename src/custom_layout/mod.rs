mod imp;

use gtk::{glib, subclass::prelude::*};

glib::wrapper! {
    pub struct CustomLayout(ObjectSubclass<imp::CustomLayout>)
        @extends gtk::LayoutManager;
}

impl CustomLayout {
    pub fn new() -> Self {
        glib::Object::new()
    }
}
