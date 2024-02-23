use std::cell::Cell;
use std::cell::RefCell;

use crate::custom_layout::CustomLayout;
use dbus::arg::AppendAll;
use glib::ObjectExt;
use glib::Properties;
use gtk::glib::clone;
use gtk::pango::AttrList;
use gtk::prelude::ToValue;
use gtk::prelude::*;
use gtk::subclass::prelude::*;
use gtk::Align;
use gtk::Box;
use gtk::Button;
use gtk::Image;
use gtk::Justification;
use gtk::MenuButton;
use gtk::Popover;
use gtk::{gio, glib, CompositeTemplate, Label};

// Object holding the state
#[derive(Default, CompositeTemplate)]
#[template(resource = "/org/gtk_rs/rusticbar/systeminfo.ui")]
pub struct SystemInfoWidget {
    pub disk: Label,
    pub disk_data: Label,
    pub ram: Label,
    pub ram_data: Label,
    pub cpu_load: Label,
    pub cpu_load_data: Label,
    pub cpu_temp: Label,
    pub cpu_temp_data: Label,
    pub uptime: Label,
    pub uptime_data: Label,
    #[template_child(id = "button")]
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
        let popover = Popover::new();
        let popover_box = Box::new(gtk::Orientation::Vertical, 10);

        let box_disk = Box::new(gtk::Orientation::Horizontal, 15);
        self.disk.set_text("DISK:");
        self.disk.set_halign(Align::Start);
        self.disk_data.set_halign(Align::End);
        box_disk.append(&self.disk);
        box_disk.append(&self.disk_data);
        box_disk.set_homogeneous(true);
        popover_box.append(&box_disk);

        let box_ram = Box::new(gtk::Orientation::Horizontal, 15);
        self.ram.set_text("RAM:");
        self.ram.set_halign(Align::Start);
        self.ram_data.set_halign(Align::End);
        box_ram.append(&self.ram);
        box_ram.append(&self.ram_data);
        box_ram.set_homogeneous(true);
        popover_box.append(&box_ram);

        let box_cpu_load = Box::new(gtk::Orientation::Horizontal, 15);
        self.cpu_load.set_text("CPU LOAD:");
        self.cpu_load.set_halign(Align::Start);
        self.cpu_load_data.set_halign(Align::End);
        box_cpu_load.append(&self.cpu_load);
        box_cpu_load.append(&self.cpu_load_data);
        box_cpu_load.set_homogeneous(true);
        popover_box.append(&box_cpu_load);

        let box_cpu_temp = Box::new(gtk::Orientation::Horizontal, 15);
        self.cpu_temp.set_text("CPU TEMP:");
        self.cpu_temp.set_halign(Align::Start);
        self.cpu_temp_data.set_halign(Align::End);
        box_cpu_temp.append(&self.cpu_temp);
        box_cpu_temp.append(&self.cpu_temp_data);
        box_cpu_temp.set_homogeneous(true);
        popover_box.append(&box_cpu_temp);

        let box_uptime = Box::new(gtk::Orientation::Horizontal, 15);
        self.uptime.set_text("UPTIME:");
        self.uptime.set_halign(Align::Start);
        self.uptime_data.set_halign(Align::End);
        box_uptime.append(&self.uptime);
        box_uptime.append(&self.uptime_data);
        box_uptime.set_homogeneous(true);
        popover_box.append(&box_uptime);

        popover.set_child(Some(&popover_box));
        popover.set_focusable(false);
        popover.set_autohide(false);

        popover.connect_closed(clone!(@weak self as widget => move |_| {
            widget.ticking.set(false);
            // widget.button.unset_create_popup_func();
            println!("connect closed");
        }));
        self.obj().call_and_set_systemstat();
        self.button.set_popover(Some(&popover));
        self.button
            .set_create_popup_func(clone!(@weak self as widget => move |_menu_button| {
                popover.popup();
                widget.obj().call_and_set_systemstat();
                widget.ticking.set(true);
                let tick_systeminfo = move || {

                    if widget.ticking.get() {
                        widget.obj().call_and_set_systemstat();
                println!("HOLA");

                        return glib::ControlFlow::Continue;
                    } else {
                        return glib::ControlFlow::Break;
                    }
                };
                glib::timeout_add_seconds_local(2, tick_systeminfo);
            }));
        // self.obj().setup_systemstat();
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
