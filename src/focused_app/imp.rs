use glib::Properties;
use gtk::pango::AttrList;
use gtk::subclass::prelude::*;
use gtk::{glib, CompositeTemplate, Label};

use crate::custom_layout::CustomLayout;

// #[derive(Default, Debug)]
// pub enum FocusedApp {
//     #[default]
//     None,
//     English,
//     Spanish,
// }

// Object holding the state
#[derive(Default, CompositeTemplate)]
#[template(resource = "/org/gtk_rs/rusticbar/focused_app.ui")]
// #[properties(wrapper_type = super::FocusedAppWidget)]
pub struct FocusedAppWidget {
    #[template_child(id = "focused_app_label")]
    pub focused_app_label: TemplateChild<Label>,
    // pub layout: FocusedApp,
}

// The central trait for subclassing a GObject
#[glib::object_subclass]
impl ObjectSubclass for FocusedAppWidget {
    // `NAME` needs to match `class` attribute of template
    const NAME: &'static str = "FocusedAppBox";
    type Type = super::FocusedAppWidget;
    type ParentType = gtk::Box;

    fn class_init(klass: &mut Self::Class) {
        klass.bind_template();
        klass.set_layout_manager_type::<CustomLayout>();
    }

    fn instance_init(obj: &glib::subclass::InitializingObject<Self>) {
        obj.init_template();
    }
}

impl FocusedAppWidget {}

// Trait shared by all GObjects
// #[glib::derived_properties]
impl ObjectImpl for FocusedAppWidget {
    fn constructed(&self) {
        self.parent_constructed();
        // self.obj().setup_layout();
    }
}

// Trait shared by all widgets
impl WidgetImpl for FocusedAppWidget {}

// Trait shared by all boxes
impl BoxImpl for FocusedAppWidget {}
