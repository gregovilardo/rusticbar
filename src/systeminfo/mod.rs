mod imp;

use glib::Object;
use gtk::{
    gio,
    glib::{self, clone},
    prelude::*,
    subclass::prelude::ObjectSubclassIsExt,
};
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

enum StatType {
    Disc,
    Ram,
    CpuLoad,
    CpuTemp,
    Uptime,
}

impl SystemInfoWidget {
    pub fn new() -> Self {
        Object::builder().build()
    }

    fn setup_popover(&self) {
        let popover = self.imp().popover.get();
        popover.connect_closed(clone!(@weak self as widget => move |_| {
            widget.imp().ticking.set(false);
        }));
        self.imp().button.set_create_popup_func(
            clone!(@weak self as widget => move |_menu_button| {
                popover.popup();
                widget.call_and_set_systemstat();
                widget.imp().ticking.set(true);
                let tick_systeminfo = move || {
                    if widget.imp().ticking.get() {
                        widget.call_and_set_systemstat();
                        return glib::ControlFlow::Continue;
                    } else {
                        return glib::ControlFlow::Break;
                    }
                };
                glib::timeout_add_seconds_local(2, tick_systeminfo);
            }),
        );
    }

    fn call_and_set_systemstat(&self) {
        let (s, r) = async_channel::unbounded();

        gio::spawn_blocking(move || {
            let sys = System::new();

            match sys.mount_at("/") {
                Ok(mount) => {
                    let _res = s.send_blocking((StatType::Disc, format!("{}", mount.avail)));
                }
                Err(x) => println!("\nMount at /: error: {}", x),
            }
            match sys.memory() {
                Ok(mem) => {
                    let _res = s.send_blocking((
                        StatType::Ram,
                        format!("{}", saturating_sub_bytes(mem.total, mem.free)),
                    ));
                }
                Err(x) => println!("\nMemory: error: {}", x),
            }

            match sys.uptime() {
                Ok(uptime) => {
                    let _res = s.send_blocking((
                        StatType::Uptime,
                        format!("{}min", (uptime.as_secs() / 60) as u32),
                    ));
                }
                Err(x) => println!("\nUptime: error: {}", x),
            }

            match sys.cpu_load_aggregate() {
                Ok(cpu) => {
                    thread::sleep(Duration::from_secs(1));
                    let cpu = cpu.done().unwrap();
                    // let prom =
                    //     (cpu.user + cpu.nice + cpu.system + cpu.interrupt + cpu.idle) * 100.0 / 5.0;
                    let _res = s.send_blocking((
                        StatType::CpuLoad,
                        format!("{}%.", (cpu.user * 100.0).round()),
                    ));
                }
                Err(x) => println!("\nCPU load: error: {}", x),
            }

            match sys.cpu_temp() {
                Ok(cpu_temp) => {
                    let _res = s.send_blocking((StatType::CpuTemp, format!("{}ºC", cpu_temp)));
                }
                Err(x) => println!("\nCPU TEMP:   {}", x),
            }
        });
        // Ofcourse there is some better ways
        glib::spawn_future_local(clone! (@weak self as widget => async move {
            while let  Ok(res) = r.recv().await {
                match res.0 {
                    StatType::Disc => {
                        widget.imp()
                            .disk_data
                            .set_text(&res.1);
                    },
                    StatType::Ram => {
                        widget.imp()
                            .ram_data
                            .set_text(&res.1);
                    },
                    StatType::CpuLoad => {
                        widget.imp()
                            .cpu_load_data
                            .set_text(&res.1);
                    },
                    StatType::CpuTemp => {
                        widget.imp()
                            .cpu_temp_data
                            .set_text(&res.1);
                    },
                    StatType::Uptime => {
                        widget.imp()
                            .uptime_data
                            .set_text(&res.1);
                    },
                }
            }
        }));
    }
}
