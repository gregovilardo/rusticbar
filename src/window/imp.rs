use std::cell::{Cell, RefCell};

use glib::subclass::InitializingObject;
use glib::Properties;
use gtk::subclass::prelude::*;
use gtk::{gio, ColumnView};
use gtk::{glib, CompositeTemplate};
use gtk::{prelude::*, Box};
use gtk::{Label, ListView};

use crate::ws_object::WsObject;
use crate::ws_widget::WsWidget;

// Object holding the state
#[derive(Properties, CompositeTemplate, Default)]
#[template(resource = "/org/gtk_rs/rusticbar/window.ui")]
#[properties(wrapper_type = super::Window)]
pub struct Window {
    #[template_child(id = "time_label")]
    pub time_label: TemplateChild<Label>,
    #[property(get, set = Self::set_time_label)]
    pub time: RefCell<String>,
    // #[property(get, set = Self::set_workspaces)]
    #[template_child(id = "wss_list")]
    pub wss_column: TemplateChild<ListView>,
    pub workspaces: RefCell<Option<gio::ListStore>>,
}

// The central trait for subclassing a GObject
#[glib::object_subclass]
impl ObjectSubclass for Window {
    // `NAME` needs to match `class` attribute of template
    const NAME: &'static str = "MyGtkAppWindow";
    type Type = super::Window;
    type ParentType = gtk::ApplicationWindow;

    fn class_init(klass: &mut Self::Class) {
        klass.bind_template();
    }

    fn instance_init(obj: &InitializingObject<Self>) {
        obj.init_template();
    }
}

impl Window {
    fn set_time_label(&self, time: String) {
        self.time_label.set_text(&time.clone());
        *self.time.borrow_mut() = time;
    }
}

#[glib::derived_properties]
impl ObjectImpl for Window {
    fn constructed(&self) {
        self.parent_constructed();
        self.obj().setup_size();
        self.obj().setup_workspaces();
        self.obj().setup_sway_events();
        self.obj().setup_factory();
    }
}

// Trait shared by all widgets
impl WidgetImpl for Window {}

// Trait shared by all windows
impl WindowImpl for Window {}

// Trait shared by all application windows
impl ApplicationWindowImpl for Window {}
