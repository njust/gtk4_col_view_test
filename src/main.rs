use gtk4::prelude::*;
use std::env::args;

mod manual_column_view;
mod manual_model;

fn build_ui(application: &gtk4::Application) {
    let window = gtk4::ApplicationWindow::new(application);

    window.set_title(Some("First GTK Program"));
    window.set_default_size(1024, 768);

    let test = manual_column_view::Test::new();

    window.set_child(Some(&test.list()));

    window.show();
}

fn main() {
    let application =
        gtk4::Application::new(Some("com.github.gtk-rs.examples.basic"), Default::default())
            .expect("Initialization failed...");

    application.connect_activate(|app| {
        build_ui(app);
    });

    application.run(&args().collect::<Vec<_>>());
}