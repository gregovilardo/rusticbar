use glib::Properties;
use gtk::prelude::*;
use gtk::subclass::prelude::*;
use gtk::{glib, CompositeTemplate, Label};
use std::cell::{Cell, Ref, RefCell};

// Object holding the state
#[derive(Properties, CompositeTemplate, Default)]
#[properties(wrapper_type = super::StatLabel)]
#[template(resource = "/org/gtk_rs/rusticbar/statlabel.ui")]
pub struct StatLabel {
    #[property(get, set)]
    pub data: Cell<f64>,
    #[property(get, set)]
    pub description: RefCell<String>,
    #[property(get, set)]
    pub unit: RefCell<String>,
    #[template_child(id = "data")]
    pub data_label: TemplateChild<Label>,
    #[template_child(id = "description")]
    pub description_label: TemplateChild<Label>,
    #[template_child(id = "unit")]
    pub unit_label: TemplateChild<Label>,
}

#[glib::object_subclass]
impl ObjectSubclass for StatLabel {
    const NAME: &'static str = "StatLabelBox";
    type Type = super::StatLabel;
    type ParentType = gtk::Box;

    fn class_init(klass: &mut Self::Class) {
        klass.bind_template();
        // klass.set_layout_manager_type::<CustomLayout>();
    }

    fn instance_init(obj: &glib::subclass::InitializingObject<Self>) {
        obj.init_template();
    }
}

// Trait shared by all GObjects
#[glib::derived_properties]
impl ObjectImpl for StatLabel {
    fn constructed(&self) {
        self.parent_constructed();

        // `SYNC_CREATE` ensures that the label will be immediately set
        let obj = self.obj();
        obj.bind_property("data", &self.data_label.get(), "label")
            .sync_create()
            .transform_to(|_, data: f64| {
                return Some(format!("{:.2}", data).to_value());
            })
            .build();

        obj.bind_property("description", &self.description_label.get(), "label")
            .sync_create()
            .build();
        obj.bind_property("unit", &self.unit_label.get(), "label")
            .sync_create()
            .build();
    }
}

// Trait shared by all widgets
impl WidgetImpl for StatLabel {}

// Trait shared by all buttons
impl BoxImpl for StatLabel {}
