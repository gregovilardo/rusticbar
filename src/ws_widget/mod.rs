mod imp;

use glib::Object;
use gtk::prelude::*;
use gtk::subclass::prelude::*;
use gtk::{glib, pango};
use pango::{AttrInt, AttrList};

use crate::ws_object::WsObject;

glib::wrapper! {
    pub struct WsWidget(ObjectSubclass<imp::WsWidget>)
    @extends gtk::Box, gtk::Widget,
    @implements gtk::Accessible, gtk::Buildable, gtk::ConstraintTarget, gtk::Orientable;
}

impl Default for WsWidget {
    fn default() -> Self {
        Self::new()
    }
}

impl WsWidget {
    pub fn new() -> Self {
        Object::builder().build()
    }

    pub fn bind(&self, ws_object: &WsObject) {
        // Get state
        let ws_label = self.imp().ws_label.get();
        let mut bindings = self.imp().bindings.borrow_mut();

        // Bind `ws_object.name` to `ws_widget.ws_label.label`
        let ws_label_binding = ws_object
            .bind_property("name", &ws_label, "label")
            .sync_create()
            .build();
        // Save binding
        bindings.push(ws_label_binding);

        // Bind `task_object.completed` to `task_row.content_label.attributes`
        let ws_label_binding = ws_object
            .bind_property("focused", &ws_label, "attributes")
            .sync_create()
            .transform_to(|_, active| {
                let attribute_list = AttrList::new();
                if active {
                    // If "active" is true, content of the label will be strikethrough
                    let attribute = AttrInt::new_strikethrough(true);
                    attribute_list.insert(attribute);
                }
                Some(attribute_list.to_value())
            })
            .build();
        // Save binding
        bindings.push(ws_label_binding);
    }

    pub fn unbind(&self) {
        // Unbind all stored bindings
        for binding in self.imp().bindings.borrow_mut().drain(..) {
            binding.unbind();
        }
    }
}
