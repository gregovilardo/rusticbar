mod imp;

use dbus::blocking::Connection;
use glib::{clone, Object};
use gtk::{gio, glib};
use passcod_networkmanager::devices::{Any, Device, Wired, Wireless};
use passcod_networkmanager::{Error, NetworkManager};

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

    fn setup_connections(&self) {
        // let dbus_connection = Connection::new_system().expect("er");

        // glib::spawn_future_local(clone!(@weak self as nw_widget => async move {
        let nm = NetworkManager::new().expect("network");
        let mut eth_conn = false;

        for dev in nm.get_all_devices().expect("er") {
            match dev {
                Device::Ethernet(x) => {
                    if let Ok(conn) = x.active_connection() {
                        if let Ok(_state) = conn.state() {
                            self.set_network_name(x.ip_interface().expect("ip interface"));
                            eth_conn = true;
                        }
                    } else {
                        println!("active connection error");
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
                }
                Device::WiFi(x) => {
                    if !eth_conn {
                        if let Ok(ap) = x.active_access_point() {
                            self.set_network_name(ap.ssid().expect("ssid"));
                        } else {
                            self.set_network_name("NO CONNECTION")
                        }
                    }
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
                }
                _ => {}
            }
        }
        // }));
    }
}
