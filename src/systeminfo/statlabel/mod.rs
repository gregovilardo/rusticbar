mod imp;

use glib::Object;
use gtk::glib;

glib::wrapper! {
    pub struct StatLabel(ObjectSubclass<imp::StatLabel>)
    @extends gtk::Box, gtk::Widget,
    @implements gtk::Accessible, gtk::Buildable, gtk::ConstraintTarget, gtk::Orientable;
}

impl StatLabel {
    pub fn new(description: &str, data: f64, unit: &str) -> Self {
        Object::builder()
            .property("description", description)
            .property("data", data)
            .property("description", unit)
            .build()
    }
}

impl Default for StatLabel {
    fn default() -> Self {
        Object::builder().build()
    }
}
