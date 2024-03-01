mod imp;

use alsa::mixer::{Mixer, Selem, SelemChannelId, SelemId};
use glib::{clone, Object};
use gtk::{
    gio, glib, prelude::*, subclass::prelude::ObjectSubclassIsExt, EventControllerMotion,
    RevealerTransitionType,
};

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
    fn setup_motions(&self) {
        let revealer = self.imp().level_bar_revealer.get();
        let event_controler = EventControllerMotion::new();

        revealer.set_transition_type(RevealerTransitionType::SlideRight);
        revealer.set_transition_duration(1000);
        revealer.set_reveal_child(false);
        let revealer_clone = revealer.clone();

        event_controler.connect_enter(move |_, _, _| {
            revealer.set_reveal_child(true);
        });
        event_controler.connect_leave(move |_| {
            revealer_clone.set_reveal_child(false);
        });

        self.imp().volume_box.add_controller(event_controler);
    }

    fn setup_volume_event(&self) {
        let mixer = Mixer::new("default", false).unwrap();
        let (sender, receiver) = async_channel::unbounded();
        let mut descriptors = alsa::poll::Descriptors::get(&mixer).expect("descriptors");

        let elem = mixer.find_selem(&SelemId::new("Master", 0)).expect("mixer");
        let status = get_status(elem);
        self.set_volume(status.0);
        self.set_mute(status.1);

        // The timeout argument specifies the number of milliseconds that
        // poll() should block waiting for a file descriptor to become
        // ready.  The call will block until either:
        gio::spawn_blocking(move || {
            while let Ok(_res) = alsa::poll::poll(&mut descriptors, i32::MAX) {
                if let Ok(r) = mixer.handle_events() {
                    if r == 1 {
                        let elem = mixer.find_selem(&SelemId::new("Master", 0)).expect("mixer");
                        let _res = sender.send_blocking(get_status(elem));
                    }
                }
            }
        });
        glib::spawn_future_local(clone! (@weak self as vol_widget => async move {
            while let Ok(status) = receiver.recv().await {
                vol_widget.set_volume(status.0);
                vol_widget.set_mute(status.1);
                // self.imp().volume_box.get().
            }
        }));
    }
}

fn get_status(selem: Selem) -> (f64, bool) {
    let mute;
    let volume = selem
        .get_playback_volume(SelemChannelId::FrontRight)
        .unwrap() as f64
        / selem.get_playback_volume_range().1 as f64
        * 100.0;
    if selem
        .get_playback_switch(SelemChannelId::FrontRight)
        .expect("mute")
        == 1
    {
        mute = false;
    } else {
        mute = true;
    }
    return (volume, mute);
}
