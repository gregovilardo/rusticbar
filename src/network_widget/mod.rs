mod imp;

use dbus::blocking::Connection;
use glib::{clone, Object};
use gtk::{gio, glib};
use networkmanager::devices::{Any, Device, Wired, Wireless};
use networkmanager::{Error, NetworkManager};

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
        let dbus_connection = Connection::new_system().expect("er");

        let nm = NetworkManager::new(&dbus_connection);

        for dev in nm.get_devices().expect("er") {
            match dev {
                Device::Ethernet(x) => {
                    // println!("Is autoconnected: {:?}", x.autoconnect().expect("er"));
                    // println!("Speed: {:?}", x.speed().expect("er"));
                    // println!("S390 Subchannels: {:?}", x.s390_subchannels().expect("er"));
                    // println!("Carrier: {:?}", x.carrier().expect("er"));
                    println!("EMMM {:?}", networkmanager::devices::Any::hw_address(&x));
                    println!("EMMM {:?}", x.interface().expect("er"));
                    println!("EMMM {:?}", x.active_connection().expect("er").state());
                    println!("EMMM {:?}", x.state().expect("er"));
                }
                Device::WiFi(x) => {
                    println!("Bitrate: {:?}", x.bitrate().expect("er1"));
                    x.request_scan(std::collections::HashMap::new())
                        .expect("er1");
                    for ap in x.get_all_access_points().expect("er1") {
                        println!("SSID: {:?}", ap.ssid().expect("er1"));
                        println!("CON {:?}", x.active_connection().expect("er").state());
                        println!("CON {:?}", x.ip6_connectivity().expect("er"));
                    }
                }
                _ => {}
            }
        }
    }
}
