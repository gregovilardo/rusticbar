use std::cell::{Cell, RefCell};

use glib::subclass::InitializingObject;
use glib::Properties;
use gtk::subclass::prelude::*;
use gtk::{gio, ColumnView};
use gtk::{glib, CompositeTemplate};
use gtk::{prelude::*, Box};
use gtk::{Label, ListView};

use crate::focused_app::FocusedAppWidget;
use crate::gammarelay::GammarelayWidget;
use crate::keyboard_layout::KeyboardLayoutWidget;
use crate::network::NetworkWidget;
use crate::systeminfo::SystemInfoWidget;
use crate::time::TimeWidget;
use crate::volume::VolWidget;

// Object holding the state
#[derive(CompositeTemplate, Default)]
#[template(resource = "/org/gtk_rs/rusticbar/window.ui")]
// #[properties(wrapper_type = super::Window)]
pub struct Window {
    #[template_child(id = "wss_list")]
    pub wss_list: TemplateChild<ListView>,
    pub workspaces: RefCell<Option<gio::ListStore>>,
    pub time_widget: TimeWidget,
    pub focused_app_widget: FocusedAppWidget,
    pub vol_widget: VolWidget,
    pub system_info_widget: SystemInfoWidget,
    pub keyboard_layout_widget: KeyboardLayoutWidget,
    pub network_widget: NetworkWidget,
    pub gammarelay_widget: GammarelayWidget,
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

impl Window {}

// #[glib::derived_properties]
impl ObjectImpl for Window {
    fn constructed(&self) {
        self.parent_constructed();
        self.obj().setup_size();
        self.obj().setup_workspaces();
        self.obj().setup_sway_events();
        self.obj().setup_factory();
        self.obj().setup_child_widgets();
        self.obj().change_workspace();
    }
}

// Trait shared by all widgets
impl WidgetImpl for Window {}

// Trait shared by all windows
impl WindowImpl for Window {}

// Trait shared by all application windows
impl ApplicationWindowImpl for Window {}
