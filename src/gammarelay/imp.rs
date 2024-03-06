use dbus::blocking::Connection;
use glib::Properties;
use gtk::pango::AttrList;
use gtk::subclass::prelude::*;
use gtk::{
    glib, traits::AdjustmentExt, traits::RangeExt, traits::ScaleExt, CompositeTemplate, Label,
    MenuButton, Popover, Scale,
};
use gtk::{Adjustment, Switch};

use crate::custom_layout::CustomLayout;

// Object holding the state
#[derive(Default, CompositeTemplate)]
#[template(resource = "/org/gtk_rs/rusticbar/gammarelay.ui")]
// #[properties(wrapper_type = super::GammarelayWidget)]
pub struct GammarelayWidget {
    #[template_child(id = "menu_button")]
    pub menu_button: TemplateChild<MenuButton>,
    #[template_child(id = "gamma_popover")]
    pub popover: TemplateChild<Popover>,
    #[template_child(id = "temp_scale")]
    pub temp_scale: TemplateChild<Scale>,
    #[template_child(id = "bright_scale")]
    pub bright_scale: TemplateChild<Scale>,
    #[template_child(id = "gamma_scale")]
    pub gamma_scale: TemplateChild<Scale>,
    #[template_child(id = "inverted_switch")]
    pub inverted_switch: TemplateChild<Switch>,
}

// The central trait for subclassing a GObject
#[glib::object_subclass]
impl ObjectSubclass for GammarelayWidget {
    // `NAME` needs to match `class` attribute of template
    const NAME: &'static str = "GammarelayBox";
    type Type = super::GammarelayWidget;
    type ParentType = gtk::Box;

    fn class_init(klass: &mut Self::Class) {
        klass.bind_template();
        klass.set_layout_manager_type::<CustomLayout>();
    }

    fn instance_init(obj: &glib::subclass::InitializingObject<Self>) {
        obj.init_template();
    }
}

impl GammarelayWidget {}

// Trait shared by all GObjects
// #[glib::derived_properties]
impl ObjectImpl for GammarelayWidget {
    fn constructed(&self) {
        self.parent_constructed();
        self.obj().setup_button();
        // let _child = match self.obj().run_wl_gammastep() {
        //     Ok(child) => child,
        //     Err(err) => {
        //         println!("{:#?}", err);
        //         return;
        //     }
        // };
        self.obj().setup_gammarelay();
        self.obj().setup_actions();
    }
}

// Trait shared by all widgets
impl WidgetImpl for GammarelayWidget {}

// Trait shared by all boxes
impl BoxImpl for GammarelayWidget {}
