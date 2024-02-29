use glib::Properties;
use gtk::pango::AttrList;
use gtk::subclass::prelude::*;
use gtk::{glib, CompositeTemplate, Label};

use crate::custom_layout::CustomLayout;

// Object holding the state
#[derive(Default, CompositeTemplate)]
#[template(resource = "/org/gtk_rs/rusticbar/time.ui")]
// #[properties(wrapper_type = super::TimeWidget)]
pub struct TimeWidget {
    #[template_child(id = "time_label")]
    pub time_label: TemplateChild<Label>,
    // pub layout: Time,
}

// The central trait for subclassing a GObject
#[glib::object_subclass]
impl ObjectSubclass for TimeWidget {
    // `NAME` needs to match `class` attribute of template
    const NAME: &'static str = "TimeBox";
    type Type = super::TimeWidget;
    type ParentType = gtk::Box;

    fn class_init(klass: &mut Self::Class) {
        klass.bind_template();
        klass.set_layout_manager_type::<CustomLayout>();
    }

    fn instance_init(obj: &glib::subclass::InitializingObject<Self>) {
        obj.init_template();
    }
}

impl TimeWidget {}

// Trait shared by all GObjects
// #[glib::derived_properties]
impl ObjectImpl for TimeWidget {
    fn constructed(&self) {
        self.parent_constructed();
        self.obj().setup_time();
    }
}

// Trait shared by all widgets
impl WidgetImpl for TimeWidget {}

// Trait shared by all boxes
impl BoxImpl for TimeWidget {}
