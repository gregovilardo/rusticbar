use std::cell::Cell;
use std::cell::RefCell;

use glib::Binding;
use glib::ObjectExt;
use glib::Properties;
use gtk::subclass::prelude::*;
use gtk::{glib, CompositeTemplate, Label, LevelBar};

use crate::custom_layout::CustomLayout;

// Object holding the state
#[derive(Properties, Default, CompositeTemplate)]
#[template(resource = "/org/gtk_rs/rusticbar/volume.ui")]
#[properties(wrapper_type = super::VolWidget)]
pub struct VolWidget {
    #[property(get, set = Self::set_volume_level_bar)]
    pub volume: Cell<f64>,
    #[template_child]
    pub vol_label: TemplateChild<Label>,
    #[template_child]
    pub vol_level_bar: TemplateChild<LevelBar>,

    #[property(get, set = Self::set_mute_label)]
    pub mute: Cell<bool>,
}

// The central trait for subclassing a GObject
#[glib::object_subclass]
impl ObjectSubclass for VolWidget {
    // `NAME` needs to match `class` attribute of template
    const NAME: &'static str = "VolumeBox";
    type Type = super::VolWidget;
    type ParentType = gtk::Box;

    fn class_init(klass: &mut Self::Class) {
        klass.bind_template();
        klass.set_layout_manager_type::<CustomLayout>();
    }

    fn instance_init(obj: &glib::subclass::InitializingObject<Self>) {
        obj.init_template();
    }
}

impl VolWidget {
    fn set_volume_level_bar(&self, volume: f64) {
        self.volume.set(volume);
        self.vol_level_bar.set_value(volume);
    }
    fn set_mute_label(&self, is_mute: bool) {
        self.mute.set(is_mute);
        if is_mute {
            self.vol_label.set_text("󰖁");
        } else {
            self.vol_label.set_text("󰕾");
        }
    }
}

// Trait shared by all GObjects
#[glib::derived_properties]
impl ObjectImpl for VolWidget {
    fn constructed(&self) {
        self.parent_constructed();
        self.obj().setup_volume_event();
    }
}

// Trait shared by all widgets
impl WidgetImpl for VolWidget {}

// Trait shared by all boxes
impl BoxImpl for VolWidget {}
