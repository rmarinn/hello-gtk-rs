use gtk::prelude::*;
use gtk::{glib, Application, ApplicationWindow, Button};

const APP_ID: &str = "org.gtk_rs.HelloWorld1";

fn main() -> glib::ExitCode {
    // create a new application
    let app = Application::builder().application_id(APP_ID).build();

    // connect to the "activate" signal of `app`
    app.connect_activate(build_ui);

    // run the application
    app.run()
}

fn build_ui(app: &Application) {
    // create a button with label and margins
    let button = Button::builder()
        .label("Press me!")
        .margin_top(12)
        .margin_bottom(12)
        .margin_start(12)
        .margin_end(12)
        .build();

    // connect to "clicked" signal of `button`
    button.connect_clicked(|button| {
        button.set_label("Hello world!");
    });

    // create a window and set the title
    let window = ApplicationWindow::builder()
        .application(app)
        .title("My Gtk App")
        .child(&button)
        .build();

    // present the window
    window.present();
}
