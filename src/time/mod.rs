mod imp;
use chrono::Local;
use glib::{clone, Object};
use gtk::{
    gio, glib, prelude::*, subclass::prelude::ObjectSubclassIsExt, EventControllerMotion, Label,
};

glib::wrapper! {
    pub struct TimeWidget(ObjectSubclass<imp::TimeWidget>)
    @extends gtk::Box, gtk::Widget,
    @implements gtk::Accessible, gtk::Buildable, gtk::ConstraintTarget, gtk::Orientable;
}

impl Default for TimeWidget {
    fn default() -> Self {
        Self::new()
    }
}

impl TimeWidget {
    pub fn new() -> Self {
        Object::builder().build()
    }
    fn setup_time(&self) {
        let time_button = self.imp().time_button.clone();

        time_button.set_label(&get_time());
        // do this because set_label renders a new label on each tick
        let time_label = time_button
            .first_child()
            .expect("toogle")
            .first_child()
            .expect("box")
            .first_child()
            .expect("label")
            .downcast_ref::<Label>()
            .expect("label ref")
            .clone();
        time_label.next_sibling().expect("arrow").set_visible(false);

        // time_button.arrow
        let tick_time = move || {
            let time = get_time();
            time_label.set_text(&time);
            glib::ControlFlow::Continue
        };

        // executes the closure once every second
        glib::timeout_add_seconds_local(1, tick_time);
    }

    // fn setup_calendar(&self) {
    //     let popover = self.imp().calendar_popover.get();
    // self.append(&popover);
    // let event_controler = EventControllerMotion::new();
    //
    // event_controler.connect_enter({
    //     let popover = popover.clone();
    //     move |_, _, _| {
    //         popover.popup();
    //     }
    // });
    // event_controler.connect_leave({
    //     let popover = popover.clone();
    //     move |_| {
    //         popover.popdown();
    //     }
    // });
    //
    // self.add_controller(event_controler);
    // println!("{:#?}", );
    // }
}

pub fn get_time() -> String {
    // format!("{}", Local::now().format("%Y-%m-%d %H:%M:%S"))
    format!("{}", Local::now().format("%H:%M:%S"))
}
