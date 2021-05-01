use gettextrs::*;
use gtk::prelude::*;
use libadwaita::subclass::prelude::*;

mod config;
mod window;
use crate::window::Window;

fn main() {
    gtk::init().unwrap_or_else(|_| panic!("Failed to initialize GTK."));
    libadwaita::init();

    setlocale(LocaleCategory::LcAll, "");
    bindtextdomain("libadwaita-test", config::LOCALEDIR);
    textdomain("libadwaita-test");

    let res = gio::Resource::load(config::PKGDATADIR.to_owned() + "/libadwaita-test.gresource")
        .expect("Could not load resources");
    gio::resources_register(&res);

    let app = gtk::Application::new(Some("nl.albering.jasper.libadwaitatest"), Default::default());
    app.connect_activate(move |app| {
        let window = Window::new();

        window.widget.set_application(Some(app));
        app.add_window(&window.widget);
        window.widget.present();
    });

    let ret = app.run();
    std::process::exit(ret);
}
