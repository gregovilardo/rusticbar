use std::cell::Cell;

use crate::custom_layout::CustomLayout;
use gtk::subclass::prelude::*;
use gtk::MenuButton;
use gtk::Popover;
use gtk::{glib, CompositeTemplate, Label};

// Object holding the state
#[derive(Default, CompositeTemplate)]
#[template(resource = "/org/gtk_rs/rusticbar/systeminfo.ui")]
pub struct SystemInfoWidget {
    // pub disk: Label,
    #[template_child(id = "disk")]
    pub disk: TemplateChild<Label>,
    #[template_child(id = "disk_data")]
    pub disk_data: TemplateChild<Label>,
    #[template_child(id = "ram")]
    pub ram: TemplateChild<Label>,
    #[template_child(id = "ram_data")]
    pub ram_data: TemplateChild<Label>,
    #[template_child(id = "cpu_load")]
    pub cpu_load: TemplateChild<Label>,
    #[template_child(id = "cpu_load_data")]
    pub cpu_load_data: TemplateChild<Label>,
    #[template_child(id = "cpu_temp")]
    pub cpu_temp: TemplateChild<Label>,
    #[template_child(id = "cpu_temp_data")]
    pub cpu_temp_data: TemplateChild<Label>,
    #[template_child(id = "uptime")]
    pub uptime: TemplateChild<Label>,
    #[template_child(id = "uptime_data")]
    pub uptime_data: TemplateChild<Label>,
    #[template_child(id = "stat_popover")]
    pub popover: TemplateChild<Popover>,
    #[template_child(id = "menu_button")]
    pub button: TemplateChild<MenuButton>,
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
        self.obj().call_and_set_systemstat();
        self.obj().setup_popover();
    }
}

// Trait shared by all widgets
impl WidgetImpl for SystemInfoWidget {}

// Trait shared by all boxes
impl BoxImpl for SystemInfoWidget {}

// popover_box.append(&self.disc);
// self.disc_icon.set_from_file(Some(
//     "/home/gregovilardo/Documents/rust/rusticbar/src/disc.png",
// ));
// self.disc_icon.set_pixel_size(30);
// let boxx = Box::new(gtk::Orientation::Horizontal, 5);
// // boxx.append(&self.disc_icon);
// boxx.append(&self.disc);
// popover_box.append(&boxx);
// self.ram_icon.set_from_file(Some(
//     "/home/gregovilardo/Documents/rust/rusticbar/src/ram2.png",
// ));
// self.ram_icon.set_pixel_size(30);
// let boxx = Box::new(gtk::Orientation::Horizontal, 5);
// // boxx.append(&self.ram_icon);
// boxx.append(&self.ram);
//
