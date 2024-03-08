mod imp;
use crate::gammarelay_dbus::RsWlGammarelay;

use dbus::blocking::{Connection, Proxy};
use glib::{clone, Object};
use gtk::EventControllerMotion;
use gtk::{gio, glib, prelude::*, subclass::prelude::ObjectSubclassIsExt};
use std::process::{Child, Command};
use std::time::Duration;

glib::wrapper! {
    pub struct GammarelayWidget(ObjectSubclass<imp::GammarelayWidget>)
    @extends gtk::Box, gtk::Widget,
    @implements gtk::Accessible, gtk::Buildable, gtk::ConstraintTarget, gtk::Orientable;
}

impl Default for GammarelayWidget {
    fn default() -> Self {
        Self::new()
    }
}

impl GammarelayWidget {
    pub fn new() -> Self {
        Object::builder().build()
    }

    fn setup_button(&self) {
        let menu_button = self.imp().menu_button.get();

        menu_button
            .first_child()
            .expect("togglebut")
            .first_child()
            .expect("gtkbox")
            .first_child()
            .expect("label")
            .next_sibling()
            .expect("builticon")
            .set_visible(false);

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
        // self.add_controller(event_controler.clone());
    }

    fn run_wl_gammastep(&self) -> Result<Child, Box<dyn std::error::Error>> {
        let mut gammarelay = Command::new("wl-gammarelay-rs");
        gammarelay.arg("run");
        let child = gammarelay.spawn()?;
        return Ok(child);
    }

    fn setup_gammarelay(&self) {
        let (s, r) = async_channel::unbounded();
        gio::spawn_blocking(move || loop {
            if let Ok(con) = Connection::new_session() {
                let _res = s.send_blocking(con);
                break;
            }
        });

        glib::spawn_future_local(clone!(@weak self as widget => async move {
            while let Ok(con) = r.recv().await {
                let _res = widget.run_wl_gammastep();
                std::thread::sleep(Duration::from_secs(5));
                let gammarelay = con.with_proxy("rs.wl-gammarelay", "/", Duration::new(5, 0));
                let widget = widget.imp();
                let temp = gammarelay.temperature().unwrap();
                widget.temp_scale.set_value(temp as f64);
                let brightness = gammarelay.brightness().unwrap();
                widget.bright_scale.set_value(brightness as f64);
                let gamma = gammarelay.gamma().unwrap();
                widget.gamma_scale.set_value(gamma as f64);
                let is_inverted = gammarelay.inverted().unwrap();
                widget.inverted_switch.set_active(is_inverted);
            }
        }));
    }

    fn setup_actions(&self) {
        let widget = self.imp();
        widget.temp_scale.connect_value_changed(move |scale| {
            let dbus_con = Connection::new_session().expect("conn");
            let gammarelay = dbus_con.with_proxy("rs.wl-gammarelay", "/", Duration::new(5, 0));
            let _res = gammarelay.set_temperature(scale.value() as u16);
        });
        widget.bright_scale.connect_value_changed(move |scale| {
            let dbus_con = Connection::new_session().expect("conn");
            let gammarelay = dbus_con.with_proxy("rs.wl-gammarelay", "/", Duration::new(5, 0));
            let _res = gammarelay.set_brightness(scale.value());
        });
        widget.gamma_scale.connect_value_changed(move |scale| {
            let dbus_con = Connection::new_session().expect("conn");
            let gammarelay = dbus_con.with_proxy("rs.wl-gammarelay", "/", Duration::new(5, 0));
            let _res = gammarelay.set_gamma(scale.value());
        });
        widget.inverted_switch.connect_active_notify(move |switch| {
            let dbus_con = Connection::new_session().expect("conn");
            let gammarelay = dbus_con.with_proxy("rs.wl-gammarelay", "/", Duration::new(5, 0));
            let _res = gammarelay.set_inverted(switch.is_active());
        });
    }
}
