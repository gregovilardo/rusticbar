use std::cell::{Cell, RefCell};

use gtk::{glib, prelude::*, subclass::prelude::*, BoxLayout, LayoutManager, Orientation};

#[derive(Debug)]
pub struct CustomLayout {}
impl Default for CustomLayout {
    fn default() -> Self {
        Self {}
    }
}

#[glib::object_subclass]
impl ObjectSubclass for CustomLayout {
    const NAME: &'static str = "CustomLayout";
    type Type = super::CustomLayout;
    type ParentType = gtk::LayoutManager;
}

impl ObjectImpl for CustomLayout {}

impl LayoutManagerImpl for CustomLayout {
    fn request_mode(&self, _widget: &gtk::Widget) -> gtk::SizeRequestMode {
        gtk::SizeRequestMode::ConstantSize
    }

    fn measure(
        &self,
        widget: &gtk::Widget,
        orientation: gtk::Orientation,
        _for_size: i32,
    ) -> (i32, i32, i32, i32) {
        let window_height = widget.root().expect("window").size_request().1;
        if orientation == Orientation::Horizontal {
            let mut min_size = 0;
            let mut nat_size = 0;

            if let Some(mut child) = widget.first_child() {
                loop {
                    println!("child {:?}", child);
                    if !child.should_layout() {
                        continue;
                    }

                    let (child_min, child_nat, _, _) = child.measure(orientation, -1);
                    min_size = min_size.max(child_min);
                    nat_size = nat_size.max(child_nat);
                    println!("{:?}", (min_size, nat_size));
                    if let Some(next_child) = child.next_sibling() {
                        child = next_child;
                    } else {
                        break;
                    }
                }
            }

            return (min_size, nat_size, -1, -1);
        } else {
            return (window_height, window_height, -1, -1);
        }
    } // Allocate the size of the widget and its children
    fn allocate(
        &self,
        // layout_manager: &Self::Type,
        widget: &gtk::Widget,
        width: i32,
        height: i32,
        _baseline: i32,
    ) {
        println!("width {:?}, height {:?}", width, height);
        let mut child = widget.first_child().unwrap();
        loop {
            if !child.should_layout() {
                continue;
            }
            child.size_allocate(&gtk::Allocation::new(0 as i32, 0 as i32, width, height), -1);
            if let Some(next_child) = child.next_sibling() {
                child = next_child;
            } else {
                break;
            }
        }
    }
}
