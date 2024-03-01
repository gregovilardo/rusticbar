mod imp;

use dbus::arg::RefArg;
use dbus::blocking::Connection;
use dbus::Message;
use glib::{clone, Object};
use gtk::subclass::prelude::ObjectSubclassIsExt;
use gtk::{gio, glib, prelude::WidgetExt};
use gtk::{EventControllerMotion, Revealer, RevealerTransitionType};
use passcod_networkmanager::devices::{Any, Device, Wired, Wireless};
use passcod_networkmanager::{Error, NetworkManager};
use std::time::Duration;

use crate::networkmanager_dbus::{
    OrgFreedesktopDBusPropertiesPropertiesChanged, OrgFreedesktopNetworkManager,
};

use self::imp::{ConnectionType, NMState};

glib::wrapper! {
    pub struct NetworkWidget(ObjectSubclass<imp::NetworkWidget>)
    @extends gtk::Box, gtk::Widget,
    @implements gtk::Accessible, gtk::Buildable, gtk::ConstraintTarget, gtk::Orientable;
}

impl Default for NetworkWidget {
    fn default() -> Self {
        Self::new()
    }
}

impl NetworkWidget {
    pub fn new() -> Self {
        Object::builder().build()
    }

    fn setup_motions(&self) {
        let revealer_label = self.imp().revealer_label.get();
        let revealer_box = self.imp().revealer_box.get();
        let event_controler = EventControllerMotion::new();

        revealer_label.set_transition_type(RevealerTransitionType::Crossfade);
        revealer_label.set_transition_duration(1000);
        revealer_label.set_reveal_child(false);
        revealer_box.set_transition_type(RevealerTransitionType::SlideRight);
        revealer_box.set_transition_duration(2000);
        revealer_box.set_reveal_child(false);

        let revealer_label_clone = revealer_label.clone();
        let revealer_box_clone = revealer_box.clone();

        event_controler.connect_enter(move |_, _, _| {
            revealer_box.set_reveal_child(true);
            revealer_label.set_reveal_child(true);
        });
        event_controler.connect_leave(move |_| {
            revealer_label_clone.set_reveal_child(false);
            revealer_box_clone.set_reveal_child(false);
        });

        self.imp().network_box.add_controller(event_controler);
    }

    fn setup_network(&self) {
        let nm = NetworkManager::new().expect("network");
        let (send, recv) = async_channel::unbounded();

        let dbus_conn = Connection::new_system().expect("dbus connection");
        let p = dbus_conn.with_proxy(
            "org.freedesktop.NetworkManager",
            "/org/freedesktop/NetworkManager",
            Duration::new(5, 0),
        );

        if let Ok(state) = p.state() {
            println!("{:?}", state);
            self.imp().state.set(translate_nmstate(state));
        }
        self.update_network(nm.clone());
        self.imp().set_icon();

        gio::spawn_blocking(clone!(@strong send => move || {
            let p = dbus_conn.with_proxy(
                "org.freedesktop.NetworkManager",
                "/org/freedesktop/NetworkManager",
                Duration::new(5, 0),
            );

            let _res = p.match_signal( move
                |state: OrgFreedesktopDBusPropertiesPropertiesChanged, _: &Connection, _: &Message| {
                    // so ugly
                    if let Some(state_box) =  state.changed_properties.get_key_value("State") {
                        if let Some(state) = state_box.1.0.as_u64() {
                            let _res = send.send_blocking(Some(state as u32));
                        }
                    }
                    let _res = send.send_blocking(None);
                    true
                },
            );

            loop {
                dbus_conn
                    .process(Duration::from_millis(1000))
                    .expect("process");
            }
        }));

        glib::spawn_future_local(clone!(@weak self as nm_widget => async move {
            while let Ok(res) = recv.recv().await {
                // when disconnecting ethernet with wifi activated no change on state happen
                if let Some(state) = res {
                    println!("{:?}", state);
                    nm_widget.imp().state.set(translate_nmstate(state));
                }
                nm_widget.update_network(nm.clone());
                nm_widget.imp().set_icon();
            }
        }));
    }
    fn update_network(&self, nm: NetworkManager) {
        for dev in nm.get_all_devices().expect("er") {
            match dev {
                Device::Ethernet(x) => {
                    if let Ok(conn) = x.active_connection() {
                        if let Ok(_state) = conn.state() {
                            self.imp().connection.set(ConnectionType::Wired);
                            // self.set_network_name(x.ip_interface().expect("ip interface"));
                            self.set_network_name("Ethernet");
                            return;
                        }
                    }
                    self.imp().connection.set(ConnectionType::None);
                }
                Device::WiFi(x) => {
                    if self.imp().connection.get() != ConnectionType::Wired {
                        if let Ok(ap) = x.active_access_point() {
                            if let Ok(ssid) = ap.ssid() {
                                self.imp().connection.set(ConnectionType::Wireless);
                                self.set_network_name(ssid);
                                return;
                            }
                        }
                        self.imp().connection.set(ConnectionType::None);
                    }
                }
                _ => {}
            }
        }
    }
}

pub fn translate_nmstate(state: u32) -> NMState {
    match state {
        0 => NMState::Unknown,
        10 => NMState::Asleep,
        20 => NMState::Disconnected,
        30 => NMState::Disconnecting,
        40 => NMState::Connecting,
        50 => NMState::ConnectedLocal,
        60 => NMState::ConnectedSite,
        70 => NMState::ConnectedGlobal,
        _ => NMState::Unknown,
    }
}

// println!("Is autoconnected: {:?}", x.autoconnect().expect("er"));
// println!("Speed: {:?}", x.speed().expect("er"));
// println!("S390 Subchannels: {:?}", x.s390_subchannels().expect("er"));
// println!("Carrier: {:?}", x.carrier().expect("er"));
// println!("EMMM {:?}", networkmanager::devices::Any::hw_address(&x));
// println!("EMMM {:?}", x.interface().expect("er"));
// println!("EMMM {:?}", x.active_connection().expect("er").state());
// println!("EMMM {:?}", x.available_connections().expect("er"));
// println!("EMMM {:?}", x.device_type().expect("er"));
// println!("Bitrate: {:?}", x.bitrate().expect("er1"));
// x.request_scan(std::collections::HashMap::new())
//     .expect("er1");
// println!("CON {:?}", x.active_connection().expect("er").state());
// println!("CON {:?}", x.ip6_connectivity().expect("er"));
// println!("CON {:?}", x.state_reason().expect("er"));
// println!("CON {:?}", x.interface().expect("er"));
// if let Ok(ap) = x.active_access_point() {
//     println!("SSID: {:?}", ap.ssid().expect("er1"));
//     println!("mode : {:?}", ap.mode().expect("er1"));
//     println!("flags: {:?}", ap.flags().expect("er1"));
//     println!("strength: {:?}", ap.strength().expect("er1"));
//     println!("wpa_flags: {:?}", ap.wpa_flags().expect("er1"));
//     println!("max_bitrate: {:?}", ap.max_bitrate().expect("er1"));
// }
