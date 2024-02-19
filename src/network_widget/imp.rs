use std::cell::Cell;
use std::cell::RefCell;

use glib::Binding;
use glib::ObjectExt;
use glib::Properties;
use gtk::subclass::prelude::*;
use gtk::{glib, CompositeTemplate, Label};

use crate::custom_layout::CustomLayout;

// Object holding the state
#[derive(Properties, Default, CompositeTemplate)]
#[template(resource = "/org/gtk_rs/rusticbar/network.ui")]
#[properties(wrapper_type = super::NetworkWidget)]
pub struct NetworkWidget {
    #[template_child(id = "network_name")]
    pub network_name: TemplateChild<Label>,
    #[template_child(id = "icon")]
    pub icon: TemplateChild<Label>,
    #[property(get, set = Self::set_label)]
    pub network_conn: Cell<bool>,
}

// The central trait for subclassing a GObject
#[glib::object_subclass]
impl ObjectSubclass for NetworkWidget {
    // `NAME` needs to match `class` attribute of template
    const NAME: &'static str = "NetworkBox";
    type Type = super::NetworkWidget;
    type ParentType = gtk::Box;

    fn class_init(klass: &mut Self::Class) {
        klass.bind_template();
        klass.set_layout_manager_type::<CustomLayout>();
    }

    fn instance_init(obj: &glib::subclass::InitializingObject<Self>) {
        obj.init_template();
    }
}

impl NetworkWidget {
    fn set_label(&self, is_connected: bool) {
        self.network_name.set_label("pepe");
    }
}

// Trait shared by all GObjects
#[glib::derived_properties]
impl ObjectImpl for NetworkWidget {
    fn constructed(&self) {
        self.parent_constructed();
        self.obj().setup_connections();
    }
}

// Trait shared by all widgets
impl WidgetImpl for NetworkWidget {}

// Trait shared by all boxes
impl BoxImpl for NetworkWidget {}
