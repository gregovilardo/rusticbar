// mod fixed_size_layout_manager;
mod custom_layout;
mod focused_app;
mod gammarelay;
mod gammarelay_dbus;
mod keyboard_layout;
mod network;
mod networkmanager_dbus;
mod systeminfo;
mod time;
mod volume;
mod window;
mod ws_object;
mod ws_widget;

// use fixed_size_layout_manager::FixedSizeLayoutManager;
use gtk::gdk::Display;
use gtk::gio::ActionEntry;
use gtk::glib::clone;
use gtk::{gio, glib, Application, CssProvider};
use gtk::{prelude::*, ApplicationWindow};
use gtk4_layer_shell as layer_shell;
use layer_shell::LayerShell;
use signal_hook::consts::signal::*;
use signal_hook::iterator::Signals;
use window::Window;

const APP_ID: &str = "org.gtk_rs.RusticBar";

fn main() -> glib::ExitCode {
    // Register and include resources
    gio::resources_register_include!("src_resources.gresource")
        .expect("Failed to register resources.");

    // Create a new applicationw
    let app = Application::builder().application_id(APP_ID).build();

    // Connect to signals
    app.connect_startup(|app| {
        load_css();
    });

    app.connect_activate(build_ui);

    // Run the application
    app.run()
}

fn load_css() {
    // Load the CSS file and add it to the provider
    let provider = CssProvider::new();
    provider.load_from_resource("/org/gtk_rs/rusticbar/style.css");

    // Add the provider to the default screen
    // gtk::style_context_remove_provider_for_display(
    //     &Display::default().expect("Could not connect to a display."),
    //     &CssProvider::default(),
    // );
    // gtk::add_prog
    gtk::style_context_add_provider_for_display(
        &Display::default().expect("Could not connect to a display."),
        &provider,
        gtk::STYLE_PROVIDER_PRIORITY_USER,
    );
}

fn build_ui(app: &Application) {
    // Create new window and present it
    let window = Window::new(app);
    window.init_layer_shell();
    window.set_layer(layer_shell::Layer::Top);
    window.set_anchor(layer_shell::Edge::Top, true);
    window.set_anchor(layer_shell::Edge::Left, true);
    window.set_anchor(layer_shell::Edge::Right, true);
    window.auto_exclusive_zone_enable();

    let (send, recv) = async_channel::unbounded();
    let mut signals = Signals::new(&[SIGUSR1]).expect("Could not register signal handler");
    gio::spawn_blocking(move || {
        for signal in signals.forever() {
            match signal {
                SIGUSR1 => {
                    let _res = send.send_blocking(true);
                }
                _ => unreachable!(),
            }
        }
    });

    glib::spawn_future_local(clone!(@weak window as window => async move {
        while let Ok(_res) = recv.recv().await {
            if window.get_visible() {
                window.set_visible(false);
            } else {
                window.set_visible(true);

            }
        }
    }));

    window.present();
}
