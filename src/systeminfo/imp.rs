use std::cell::Cell;
use std::cell::RefCell;

use glib::ObjectExt;
use glib::Properties;
use gtk::pango::AttrList;
use gtk::subclass::prelude::*;
use gtk::{glib, CompositeTemplate, Label};

use crate::custom_layout::CustomLayout;

// Object holding the state
#[derive(Default, CompositeTemplate)]
#[template(resource = "/org/gtk_rs/rusticbar/systeminfo.ui")]
pub struct SystemInfoWidget {
    #[template_child(id = "disc")]
    pub disc: TemplateChild<Label>,
    #[template_child(id = "ram")]
    pub ram: TemplateChild<Label>,
    // #[template_child(id = "swap")]
    // pub swap: TemplateChild<Label>,
    #[template_child(id = "cpu_load")]
    pub cpu_load: TemplateChild<Label>,
    #[template_child(id = "cpu_temp")]
    pub cpu_temp: TemplateChild<Label>,
    #[template_child(id = "uptime")]
    pub uptime: TemplateChild<Label>,
}

// The central trait for subclassing a GObject
#[glib::object_subclass]
impl ObjectSubclass for SystemInfoWidget {
    // `NAME` needs to match `class` attribute of template
    const NAME: &'static str = "SystemInfoBox";
    type Type = super::SystemInfoWidget;
    type ParentType = gtk::Box;

    fn class_init(klass: &mut Self::Class) {
        klass.bind_template();
        klass.set_layout_manager_type::<CustomLayout>();
    }

    fn instance_init(obj: &glib::subclass::InitializingObject<Self>) {
        obj.init_template();
    }
}

impl SystemInfoWidget {}

// Trait shared by all GObjects
impl ObjectImpl for SystemInfoWidget {
    fn constructed(&self) {
        self.parent_constructed();
        self.obj().setup_systemstat();
        self.obj().setup_tick();
    }
}

// Trait shared by all widgets
impl WidgetImpl for SystemInfoWidget {}

// Trait shared by all boxes
impl BoxImpl for SystemInfoWidget {}
