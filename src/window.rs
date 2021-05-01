use gtk::prelude::*;

pub struct Window {
    pub widget: gtk::ApplicationWindow,
}

impl Window {

    pub fn new() -> Self {
        let builder = gtk::Builder::from_resource("/nl/albering/jasper/libadwaitatest/window.ui");

        let widget = builder.object::<gtk::ApplicationWindow>("window").unwrap();

        Self { widget }
    }
}
