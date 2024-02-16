mod imp;

use alsa::mixer::{Mixer, Selem, SelemChannelId, SelemId};
use glib::{clone, Object};
use gtk::{gio, glib};

glib::wrapper! {
    pub struct VolWidget(ObjectSubclass<imp::VolWidget>)
    @extends gtk::Box, gtk::Widget,
    @implements gtk::Accessible, gtk::Buildable, gtk::ConstraintTarget, gtk::Orientable;
}

impl Default for VolWidget {
    fn default() -> Self {
        Self::new()
    }
}

impl VolWidget {
    pub fn new() -> Self {
        Object::builder().build()
    }

    fn setup_volume_event(&self) {
        let mixer = Mixer::new("default", false).unwrap();
        let (sender, receiver) = async_channel::unbounded();
        let mut descriptors = alsa::poll::Descriptors::get(&mixer).expect("descriptors");

        let elem = mixer.find_selem(&SelemId::new("Master", 0)).expect("mixer");
        self.set_volume(get_vol(elem) as f64 / 655.36);

        // The timeout argument specifies the number of milliseconds that
        // poll() should block waiting for a file descriptor to become
        // ready.  The call will block until either:
        gio::spawn_blocking(move || loop {
            let _res = alsa::poll::poll(&mut descriptors, i32::MAX);
            if let Ok(r) = mixer.handle_events() {
                if r == 1 {
                    let elem = mixer.find_selem(&SelemId::new("Master", 0)).expect("mixer");
                    let _res = sender.send_blocking(get_vol(elem));
                }
            }
        });
        glib::spawn_future_local(clone! (@weak self as vol_widget => async move {
            while let Ok(vol) = receiver.recv().await {
                // println!("Current volume alsa: {:?}", vol);
                // println!("Current volume: {:?}", vol as f64/655.36);
                vol_widget.set_volume(vol as f64/655.36);
            }
        }));
    }
}

fn get_vol(selem: Selem) -> i64 {
    let volume = selem
        .get_playback_volume(SelemChannelId::FrontRight)
        .unwrap();
    return volume;
}
