mod imp;

use glib::Object;
use gtk::{glib, subclass::prelude::ObjectSubclassIsExt};
use std::thread;
use std::time::Duration;
use systemstat::{saturating_sub_bytes, Platform, System};

glib::wrapper! {
    pub struct SystemInfoWidget(ObjectSubclass<imp::SystemInfoWidget>)
    @extends gtk::Box, gtk::Widget,
    @implements gtk::Accessible, gtk::Buildable, gtk::ConstraintTarget, gtk::Orientable;
}

impl Default for SystemInfoWidget {
    fn default() -> Self {
        Self::new()
    }
}

impl SystemInfoWidget {
    pub fn new() -> Self {
        Object::builder().build()
    }
    pub fn setup_systemstat(&self) {
        let sys = System::new();

        match sys.mount_at("/") {
            Ok(mount) => {
                self.imp()
                    .disc
                    .set_text(format!(" {}", mount.avail.to_string()).as_str());
            }
            Err(x) => println!("\nMount at /: error: {}", x),
        }

        // match sys.battery_life() {
        //     Ok(battery) => print!(
        //         "\nBattery: {}%, {}h{}m remaining",
        //         battery.remaining_capacity * 100.0,
        //         battery.remaining_time.as_secs() / 3600,
        //         battery.remaining_time.as_secs() % 60
        //     ),
        //     Err(x) => print!("\nBattery: error: {}", x),
        // }

        // match sys.on_ac_power() {
        //     Ok(power) => println!(", AC power: {}", power),
        //     Err(x) => println!(", AC power: error: {}", x),
        // }

        match sys.memory() {
            Ok(mem) => {
                self.imp()
                    .ram
                    .set_text(format!(" {}", saturating_sub_bytes(mem.total, mem.free)).as_str());
            }
            Err(x) => println!("\nMemory: error: {}", x),
        }

        // match sys.swap() {
        //     Ok(swap) => {
        //         self.imp().swap.set_text(
        //             format!("Swap: {}", saturating_sub_bytes(swap.total, swap.free)).as_str(),
        //         );
        //     }
        //     Err(x) => println!("\nSwap: error: {}", x),
        // }

        match sys.uptime() {
            Ok(uptime) => self
                .imp()
                .uptime
                .set_text(format!("  {}min", (uptime.as_secs() / 60) as u32).as_str()),
            Err(x) => println!("\nUptime: error: {}", x),
        }

        // match sys.boot_time() {
        //     Ok(boot_time) => println!("\nBoot time: {}", boot_time),
        //     Err(x) => println!("\nBoot time: error: {}", x),
        // }

        match sys.cpu_load_aggregate() {
            Ok(cpu) => {
                thread::sleep(Duration::from_secs(1));
                let cpu = cpu.done().unwrap();
                // let prom =
                //     (cpu.user + cpu.nice + cpu.system + cpu.interrupt + cpu.idle) * 100.0 / 5.0;
                self.imp()
                    .cpu_load
                    .set_text(format!("   {}%.", (cpu.user * 100.0).round()).as_str());
            }
            Err(x) => println!("\nCPU load: error: {}", x),
        }

        match sys.cpu_temp() {
            Ok(cpu_temp) => self
                .imp()
                .cpu_temp
                .set_text(format!("󰏈   {}", cpu_temp).as_str()),
            Err(x) => println!("\nCPU temp: {}", x),
        }
    }
    fn setup_tick(&self) {
        let system_widget = self.clone();
        let tick_systeminfo = move || {
            system_widget.setup_systemstat();
            // we could return glib::ControlFlow::Break to stop our clock after this tick
            glib::ControlFlow::Continue
        };
        glib::timeout_add_seconds_local(5, tick_systeminfo);
    }
}
