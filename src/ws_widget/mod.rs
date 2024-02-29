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
        let ws_label = self.imp().ws_label.get(); // ws_label is in WsWidget
        let mut bindings = self.imp().bindings.borrow_mut();

        let ws_label_binding = ws_object
            .bind_property("name", &ws_label, "label")
            .sync_create()
            .build();
        bindings.push(ws_label_binding);

        let ws_att_binding = ws_object
            .bind_property("focused", &ws_label, "attributes")
            .sync_create()
            .transform_to(|_, is_focused| {
                let attribute_list = AttrList::new();
                if is_focused {
                    let attribute = AttrInt::new_weight(pango::Weight::Ultrabold);
                    attribute_list.insert(attribute);
                }
                Some(attribute_list.to_value())
            })
            .build();
        bindings.push(ws_att_binding);

        //This is for the css
        let ws_name_binding = ws_object
            .bind_property("focused", &ws_label, "name")
            .sync_create()
            .transform_to(|_, is_focused| {
                let mut widget_name = "ws_unfocused".to_string();
                if is_focused {
                    widget_name = "ws_focused".to_string();
                }
                Some(widget_name.to_value())
            })
            .build();
        bindings.push(ws_name_binding);
    }

    pub fn unbind(&self) {
        // Unbind all stored bindings
        for binding in self.imp().bindings.borrow_mut().drain(..) {
            binding.unbind();
        }
    }
}
