use glib::Properties;
use gtk::pango::AttrList;
use gtk::subclass::prelude::*;
use gtk::{glib, CompositeTemplate, Label};

use crate::custom_layout::CustomLayout;

// Object holding the state
#[derive(Default, CompositeTemplate)]
#[template(resource = "/org/gtk_rs/rusticbar/gammarelay.ui")]
// #[properties(wrapper_type = super::GammarelayWidget)]
pub struct GammarelayWidget {
    #[template_child(id = "gammarelay_label")]
    pub gammarelay_label: TemplateChild<Label>,
    // pub layout: Gammarelay,
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
        self.obj().run_gammarelay();
    }
}

// Trait shared by all widgets
impl WidgetImpl for GammarelayWidget {}

// Trait shared by all boxes
impl BoxImpl for GammarelayWidget {}
