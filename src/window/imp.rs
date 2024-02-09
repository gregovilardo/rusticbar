use std::cell::{Cell, RefCell};

use glib::subclass::InitializingObject;
use glib::Properties;
use gtk::subclass::prelude::*;
use gtk::Label;
use gtk::{glib, CompositeTemplate};
use gtk::{prelude::*, Box};

// Object holding the state
#[derive(Properties, CompositeTemplate, Default)]
#[template(resource = "/org/gtk_rs/rusticbar/window.ui")]
#[properties(wrapper_type = super::Window)]
pub struct Window {
    #[template_child(id = "workspaces_box")]
    pub workspaces_box: TemplateChild<Box>,
    #[template_child(id = "time_label")]
    pub time_label: TemplateChild<Label>,
    #[property(get, set = Self::set_time_label)]
    pub time: RefCell<String>,
    // #[property(get, set = Self::set_workspaces)]
    pub workspaces: Vec<String>,
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
    // fn set_workspaces(&self, )
}

#[glib::derived_properties]
impl ObjectImpl for Window {
    // fn signals() -> &'static [glib::subclass::Signal] {
    //     static SIGNALS: OnceLock<Vec<Signal>> = OnceLock::new();
    //     SIGNALS.get_or_init(|| {
    //         vec![Signal::builder("clock-tick")
    //             .param_types([i32::static_type()])
    //             .build()]
    //     })
    // }

    fn constructed(&self) {
        // Call "constructed" on parent
        self.parent_constructed();
        self.obj().setup_size();
        // self.workspaces_box.append(&Label::new(Some("1")))
        // self.obj()
        //     .bind_property("time", self.obj().as_ref(), "time_label")
        //     .sync_create()
        //     .build();
    }
}

// Trait shared by all widgets
impl WidgetImpl for Window {}

// Trait shared by all windows
impl WindowImpl for Window {}

// Trait shared by all application windows
impl ApplicationWindowImpl for Window {}
