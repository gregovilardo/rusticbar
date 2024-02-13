use std::cell::RefCell;

use glib::Binding;
use gtk::subclass::prelude::*;
use gtk::{glib, CompositeTemplate, Label};

use crate::custom_layout::CustomLayout;

// Object holding the state
#[derive(Default, CompositeTemplate)]
#[template(resource = "/org/gtk_rs/rusticbar/workspace.ui")]
pub struct WsWidget {
    #[template_child]
    pub ws_label: TemplateChild<Label>,
    pub bindings: RefCell<Vec<Binding>>,
}

// The central trait for subclassing a GObject
#[glib::object_subclass]
impl ObjectSubclass for WsWidget {
    // `NAME` needs to match `class` attribute of template
    const NAME: &'static str = "WorkspaceBox";
    type Type = super::WsWidget;
    type ParentType = gtk::Box;

    fn class_init(klass: &mut Self::Class) {
        klass.bind_template();
        klass.set_layout_manager_type::<CustomLayout>();
    }

    fn instance_init(obj: &glib::subclass::InitializingObject<Self>) {
        obj.init_template();
    }
}

// Trait shared by all GObjects
impl ObjectImpl for WsWidget {
    fn constructed(&self) {
        self.parent_constructed();
    }
}

// Trait shared by all widgets
impl WidgetImpl for WsWidget {}

// Trait shared by all boxes
impl BoxImpl for WsWidget {}
