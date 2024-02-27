mod imp;
use glib::{clone, Object};
use gtk::{gio, glib, subclass::prelude::ObjectSubclassIsExt};
use swayipc::{self, Fallible, Input, InputEvent};

glib::wrapper! {
    pub struct KeyboardLayoutWidget(ObjectSubclass<imp::KeyboardLayoutWidget>)
    @extends gtk::Box, gtk::Widget,
    @implements gtk::Accessible, gtk::Buildable, gtk::ConstraintTarget, gtk::Orientable;
}

impl Default for KeyboardLayoutWidget {
    fn default() -> Self {
        Self::new()
    }
}

impl KeyboardLayoutWidget {
    pub fn new() -> Self {
        Object::builder().build()
    }
    //TODO:How to get the abbreviation
    pub fn layout_from_sway_ev(&self, ev: std::boxed::Box<InputEvent>) {
        if let Some(new_k_layout) = ev.input.xkb_active_layout_name {
            self.imp().keyboard_layout_label.set_text(&new_k_layout);
        }
    }

    pub fn setup_layout(&self, inputs: Fallible<Vec<Input>>) {
        if let Ok(inputs) = inputs {
            for input in inputs.into_iter() {
                if input.input_type == "keyboard" {
                    if let Some(k_layout) = input.xkb_active_layout_name {
                        self.imp().keyboard_layout_label.set_text(&k_layout);
                    }
                }
            }
        }
    }
}
