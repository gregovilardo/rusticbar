use gtk::glib;
use gtk::subclass::prelude::*;

// Object holding the state
#[derive(Default)]
pub struct TimeLabel;

// The central trait for subclassing a GObject
#[glib::object_subclass]
impl ObjectSubclass for TimeLabel {
    const NAME: &'static str = "MyGtkAppTimeLabel";
    type Type = super::TimeLabel;
    type ParentType = gtk::Label;
}

// Trait shared by all GObjects
impl ObjectImpl for CustomButton {}

// Trait shared by all widgets
impl WidgetImpl for CustomButton {}

// Trait shared by all buttons
impl ButtonImpl for CustomButton {}
