mod imp;
mod statlabel;

use glib::Object;
use gtk::{
    gio,
    glib::{self, clone},
    prelude::*,
    subclass::prelude::ObjectSubclassIsExt,
    Box, EventControllerMotion,
};
use std::thread;
use std::time::Duration;
use systemstat::{saturating_sub_bytes, Platform, System};

use self::statlabel::StatLabel;

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

    fn setup_box(&self) {
        let widget = self.imp();
        let stats: Vec<&StatLabel> = vec![
            &widget.disk,
            &widget.ram,
            &widget.cpu_load,
            &widget.cpu_temp,
            &widget.uptime,
        ];

        let desc: Vec<&str> = vec!["DISC:", "RAM:", "CPU:", "CPU_TEMP:", "UPTIME:"];
        let units: Vec<&str> = vec!["GB", "GB", "%", "Cº", ""];
        for ((stat, description), unit) in stats.iter().zip(desc).zip(units) {
            widget.stat_box.append(*stat);
            stat.set_description(description);
            stat.set_unit(unit);
        }
    }

    fn setup_popover(&self) {
        let popover = self.imp().popover.get();
        let event_controler = EventControllerMotion::new();

        event_controler.connect_enter({
            let popover = popover.clone();
            move |_, _, _| {
                popover.popup();
            }
        });
        event_controler.connect_leave({
            let popover = popover.clone();
            move |_| {
                popover.popdown();
            }
        });

        self.add_controller(event_controler);

        popover.connect_closed(clone!(@weak self as widget => move |_| {
            widget.imp().ticking.set(false);
        }));

        popover.connect_show(clone!(@weak self as widget => move |_popover| {
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
        }));
    }

    fn call_and_set_systemstat(&self) {
        let (s, r) = async_channel::unbounded();

        gio::spawn_blocking(move || {
            let sys = System::new();

            match sys.mount_at("/") {
                Ok(mount) => {
                    let _res = s.send_blocking((
                        StatType::Disc,
                        (mount.avail.as_u64() as f64 / 1000_f64.powf(3.0)),
                    ));
                }
                Err(x) => println!("\nMount at /: error: {}", x),
            }
            match sys.memory() {
                Ok(mem) => {
                    let _res = s.send_blocking((
                        StatType::Ram,
                        saturating_sub_bytes(mem.total, mem.free).as_u64() as f64
                            / 1000_f64.powf(3.0),
                    ));
                }
                Err(x) => println!("\nMemory: error: {}", x),
            }

            match sys.uptime() {
                Ok(uptime) => {
                    let _res = s.send_blocking((StatType::Uptime, (uptime.as_secs() / 60) as f64));
                }
                Err(x) => println!("\nUptime: error: {}", x),
            }

            match sys.cpu_load_aggregate() {
                Ok(cpu) => {
                    thread::sleep(Duration::from_secs(1));
                    let cpu = cpu.done().unwrap();
                    // let prom =
                    //     (cpu.user + cpu.nice + cpu.system + cpu.interrupt + cpu.idle) * 100.0 / 5.0;
                    let _res = s.send_blocking((StatType::CpuLoad, (cpu.user * 100.0) as f64));
                }
                Err(x) => println!("\nCPU load: error: {}", x),
            }

            match sys.cpu_temp() {
                Ok(cpu_temp) => {
                    let _res = s.send_blocking((StatType::CpuTemp, cpu_temp as f64));
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
                            .disk.set_data(res.1);
                    },
                    StatType::Ram => {
                        widget.imp()
                            .ram
                            .set_data(res.1);
                    },
                    StatType::CpuLoad => {
                        widget.imp()
                            .cpu_load
                            .set_data(res.1);
                    },
                    StatType::CpuTemp => {
                        widget.imp()
                            .cpu_temp
                            .set_data(res.1);
                    },
                    StatType::Uptime => {
                        widget.imp()
                            .uptime
                            .set_data(res.1);
                    },
                }
            }
        }));
    }
}
