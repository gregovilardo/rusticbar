use glib::Properties;
use gtk::pango::AttrList;
use gtk::subclass::prelude::*;
use gtk::{glib, CompositeTemplate, Label};

use crate::custom_layout::CustomLayout;

// #[derive(Default, Debug)]
// pub enum KeyboardLayout {
//     #[default]
//     None,
//     English,
//     Spanish,
// }

// Object holding the state
#[derive(Default, CompositeTemplate)]
#[template(resource = "/org/gtk_rs/rusticbar/keyboard_layout.ui")]
// #[properties(wrapper_type = super::KeyboardLayoutWidget)]
pub struct KeyboardLayoutWidget {
    #[template_child(id = "keyboard_layout_label")]
    pub keyboard_layout_label: TemplateChild<Label>,
    // pub layout: KeyboardLayout,
}

// The central trait for subclassing a GObject
#[glib::object_subclass]
impl ObjectSubclass for KeyboardLayoutWidget {
    // `NAME` needs to match `class` attribute of template
    const NAME: &'static str = "KeyboardLayoutBox";
    type Type = super::KeyboardLayoutWidget;
    type ParentType = gtk::Box;

    fn class_init(klass: &mut Self::Class) {
        klass.bind_template();
        klass.set_layout_manager_type::<CustomLayout>();
    }

    fn instance_init(obj: &glib::subclass::InitializingObject<Self>) {
        obj.init_template();
    }
}

impl KeyboardLayoutWidget {}

// Trait shared by all GObjects
// #[glib::derived_properties]
impl ObjectImpl for KeyboardLayoutWidget {
    fn constructed(&self) {
        self.parent_constructed();
        // self.obj().setup_layout();
    }
}

// Trait shared by all widgets
impl WidgetImpl for KeyboardLayoutWidget {}

// Trait shared by all boxes
impl BoxImpl for KeyboardLayoutWidget {}
