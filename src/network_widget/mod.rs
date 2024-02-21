mod imp;

use dbus::blocking::Connection;
use dbus::Message;
use glib::{clone, Object};
use gtk::{gio, glib};
use passcod_networkmanager::devices::{Any, Device, Wired, Wireless};
use passcod_networkmanager::{Error, NetworkManager};
use std::time::Duration;

use crate::networkmanager::OrgFreedesktopNetworkManagerStateChanged;

const NM_DEVICE_STATE_ACTIVATED: i32 = 100;

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
    fn setup_network(&self) {
        let (send, recv) = async_channel::unbounded();

        gio::spawn_blocking(clone!(@strong send => move || {
            let dbus_conn = Connection::new_system().expect("dbus connection");

            // Make a "ConnPath" struct that just contains a Connection, a destination and a path.
            let p = dbus_conn.with_proxy(
                "org.freedesktop.NetworkManager",
                "/org/freedesktop/NetworkManager",
                Duration::new(5, 0),
            );

            let _res = p.match_signal( move
                |state: OrgFreedesktopNetworkManagerStateChanged, _: &Connection, _: &Message| {
                    // println!("sig {:?}", h);
                    let _res = send.send_blocking(state);
                    true
                },
            );

            loop {
                dbus_conn
                    .process(Duration::from_millis(1000))
                    .expect("process");
            }
        }));

        glib::spawn_future_local(clone!(@weak self as nw_widget => async move {
            while let Ok(res) = recv.recv().await {
                println!("{:?}", res);
                let nm = NetworkManager::new().expect("network");
                let mut eth_conn = false;

                for dev in nm.get_all_devices().expect("er") {
                    match dev {
                        Device::Ethernet(x) => {
                            if let Ok(conn) = x.active_connection() {
                                if let Ok(_state) = conn.state() {
                                    nw_widget.set_network_name(x.ip_interface().expect("ip interface"));
                                    eth_conn = true;
                                }
                            } else {
                                println!("active connection error");
                            }
                        }
                        Device::WiFi(x) => {
                            if !eth_conn {
                                if let Ok(ap) = x.active_access_point() {
                                    nw_widget.set_network_name(ap.ssid().expect("ssid"));
                                } else {
                                    nw_widget.set_network_name("NO CONNECTION")
                                }
                            }
                        }
                        _ => {}
                    }
                }
            }
        }));
    }
    // fn setup_network(&self) {
    //     self.update_network();
    // }
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
