// mod fixed_size_layout_manager;
mod custom_layout;
mod network_widget;
mod networkmanager;
mod systeminfo;
mod vol_widget;
mod window;
mod ws_object;
mod ws_widget;

// use fixed_size_layout_manager::FixedSizeLayoutManager;
use gtk::gdk::Display;
use gtk::prelude::*;
use gtk::{gio, glib, Application, CssProvider};
use gtk4_layer_shell as layer_shell;
use layer_shell::LayerShell;
use window::{get_time, Window};

use swayipc::{self};

const APP_ID: &str = "org.gtk_rs.RusticBar";

fn main() -> glib::ExitCode {
    // Register and include resources
    gio::resources_register_include!("src_resources.gresource")
        .expect("Failed to register resources.");

    // Create a new applicationw
    let app = Application::builder().application_id(APP_ID).build();

    // Connect to signals
    app.connect_startup(|_app| {
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
    gtk::style_context_add_provider_for_display(
        &Display::default().expect("Could not connect to a display."),
        &provider,
        gtk::STYLE_PROVIDER_PRIORITY_APPLICATION,
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

    window.present();

    let tick_time = move || {
        let time = get_time();
        window.set_time(time);
        // we could return glib::ControlFlow::Break to stop our clock after this tick
        glib::ControlFlow::Continue
    };

    // executes the closure once every second
    glib::timeout_add_seconds_local(1, tick_time);
}
