use std::cell::Cell;
use std::cell::RefCell;

use glib::ObjectExt;
use glib::Properties;
use gtk::pango::AttrList;
use gtk::subclass::prelude::*;
use gtk::Box;
use gtk::Revealer;
use gtk::{glib, CompositeTemplate, Label};

use crate::custom_layout::CustomLayout;

#[derive(glib::Enum, Clone, Copy, Debug, PartialEq, Default)]
#[enum_type(name = "ConnectionType")]
pub enum ConnectionType {
    #[default]
    None,
    Wireless,
    Wired,
}

#[derive(glib::Enum, Clone, Copy, Debug, PartialEq, Default)]
#[enum_type(name = "NMState")]
pub enum NMState {
    #[default]
    Unknown,
    Asleep,
    Disconnected,
    Disconnecting,
    Connecting,
    ConnectedLocal,
    ConnectedSite,
    ConnectedGlobal,
}

// Object holding the state
#[derive(Properties, Default, CompositeTemplate)]
#[template(resource = "/org/gtk_rs/rusticbar/network.ui")]
#[properties(wrapper_type = super::NetworkWidget)]
pub struct NetworkWidget {
    #[template_child(id = "network_name")]
    pub network_name_label: TemplateChild<Label>,
    #[template_child(id = "icon")]
    pub icon: TemplateChild<Label>,
    #[template_child(id = "network_box")]
    pub network_box: TemplateChild<Box>,
    #[property(get, set = Self::set_name)]
    pub network_name: RefCell<String>,
    pub state: Cell<NMState>,
    pub connection: Cell<ConnectionType>,
    #[template_child(id = "network_revealer_label")]
    pub revealer_label: TemplateChild<Revealer>,
    #[template_child(id = "network_revealer_box")]
    pub revealer_box: TemplateChild<Revealer>,
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
    fn set_name(&self, name: String) {
        self.network_name.set(name.clone());
        self.network_name_label.set_label(&name);
    }
    pub fn set_icon(&self) {
        match self.state.get() {
            NMState::ConnectedGlobal => {
                self.network_name_label.set_attributes(Some(
                    &AttrList::from_string("0 -1 weight 500").expect("att"),
                ));
                match self.connection.get() {
                    ConnectionType::Wireless => {
                        self.icon.set_label("󰤥");
                    }
                    ConnectionType::Wired => {
                        self.icon.set_label("󰈀");
                    }
                    ConnectionType::None => {
                        println!("Something went wrong");
                    }
                }
            }
            NMState::Disconnected => {
                self.set_name("".to_string());
                self.icon.set_label("󰤮");
            }
            _ => {
                self.icon.set_label("󰤦");
            }
        }
    }
}

// Trait shared by all GObjects
#[glib::derived_properties]
impl ObjectImpl for NetworkWidget {
    fn constructed(&self) {
        self.parent_constructed();
        self.obj().setup_network();
        self.obj().setup_motions();
    }
}

// Trait shared by all widgets
impl WidgetImpl for NetworkWidget {}

// Trait shared by all boxes
impl BoxImpl for NetworkWidget {}
