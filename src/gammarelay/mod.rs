mod imp;
use crate::gammarelay_dbus::{OrgFreedesktopDBusPropertiesPropertiesChanged, RsWlGammarelay};

use dbus::blocking::Connection;
use glib::{clone, Object};
use gtk::{
    glib, subclass::prelude::ObjectSubclassIsExt, traits::AdjustmentExt, traits::ButtonExt,
    traits::RangeExt, traits::ScaleExt, CompositeTemplate, Label, MenuButton, Popover, Scale,
};
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

    fn run_wl_gammastep(&self) -> Result<Child, Box<dyn std::error::Error>> {
        let mut gammarelay = Command::new("wl-gammarelay-rs");
        gammarelay.arg("run");
        let mut child = gammarelay.spawn()?;
        std::thread::sleep(Duration::from_secs(1));
        return Ok(child);
    }

    fn setup_gammarelay(&self) {
        let dbus_con = Connection::new_session().expect("conn");
        let gammarelay = dbus_con.with_proxy("rs.wl-gammarelay", "/", Duration::new(5, 0));
        let widget = self.imp();
        match gammarelay.temperature() {
            Ok(temp) => widget.temp_scale.set_value(temp as f64),
            Err(err) => {
                eprintln!("{:#?}", err);
            }
        };
        match gammarelay.brightness() {
            Ok(brightness) => widget.bright_scale.set_value(brightness as f64),
            Err(err) => {
                eprintln!("{:#?}", err);
            }
        };
        match gammarelay.gamma() {
            Ok(gamma) => widget.gamma_scale.set_value(gamma as f64),
            Err(err) => {
                eprintln!("{:#?}", err);
            }
        };
        match gammarelay.inverted() {
            Ok(is_inverted) => widget.inverted_switch.set_active(is_inverted),
            Err(err) => {
                eprintln!("{:#?}", err);
            }
        }
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
