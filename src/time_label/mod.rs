mod imp;

use glib::Object;
use gtk::glib;

glib::wrapper! {
    pub struct TimeLabel(ObjectSubclass<imp::TimeLabel>)
        @extends gtk::Label, gtk::Widget,
        @implements gtk::Accessible, gtk::Buildable, gtk::ConstraintTarget;
}

impl TimeLabel {
    pub fn new() -> Self {
        Object::builder().build()
    }
}
