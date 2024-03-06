use std::cell::Cell;

use crate::custom_layout::CustomLayout;
use gtk::pango::AttrList;
use gtk::subclass::prelude::*;
use gtk::{prelude::*, Box};
// use gtk::MenuButton;
use gtk::Popover;
use gtk::{glib, CompositeTemplate, Label};

use super::statlabel::StatLabel;

// Object holding the state
#[derive(Default, CompositeTemplate)]
#[template(resource = "/org/gtk_rs/rusticbar/systeminfo.ui")]
pub struct SystemInfoWidget {
    pub disk: StatLabel,
    pub ram: StatLabel,
    pub cpu_load: StatLabel,
    pub cpu_temp: StatLabel,
    pub uptime: StatLabel,
    #[template_child(id = "stat_box")]
    pub stat_box: TemplateChild<Box>,
    #[template_child(id = "stat_popover")]
    pub popover: TemplateChild<Popover>,
    pub ticking: Cell<bool>,
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

#[gtk::template_callbacks]
impl SystemInfoWidget {}

// Trait shared by all GObjects
impl ObjectImpl for SystemInfoWidget {
    fn constructed(&self) {
        self.parent_constructed();
        self.obj().setup_box();
        self.obj().call_and_set_systemstat();
        self.obj().setup_popover();
        self.ram
            .bind_property("data", &self.ram.imp().data_label.get(), "name")
            .sync_create()
            .transform_to(|_, data: f64| {
                let mut widget_name = "stat_normal".to_string();
                if data > 7.0 {
                    widget_name = "stat_high".to_string();
                } else if data > 4.0 {
                    widget_name = "stat_medium".to_string();
                }
                Some(widget_name.to_value())
            })
            .build();
        self.cpu_load
            .bind_property("data", &self.cpu_load.imp().data_label.get(), "name")
            .sync_create()
            .transform_to(|_, data: f64| {
                let mut widget_name = "stat_normal".to_string();
                if data > 80.0 {
                    widget_name = "stat_high".to_string();
                } else if data > 40.0 {
                    widget_name = "stat_medium".to_string();
                }
                Some(widget_name.to_value())
            })
            .build();
    }
}

// Trait shared by all widgets
impl WidgetImpl for SystemInfoWidget {}

// Trait shared by all boxes
impl BoxImpl for SystemInfoWidget {}
