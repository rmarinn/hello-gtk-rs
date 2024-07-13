mod custom;

use custom::CounterButton;
use gtk::prelude::*;
use gtk::{glib, glib::clone, Application, ApplicationWindow, Button, Label};
use std::cell::Cell;

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
    // ----- basic button with references to other GObjects -----

    // create a label
    let counter_lbl = Label::builder()
        .label("You pressed the button 0 times!")
        .build();

    // create a button
    let btn = Button::builder().label("Click me!").margin_top(6).build();

    // a mutable integer
    let number: Cell<usize> = Cell::new(0);

    // connect callbacks
    btn.connect_clicked(clone!(
        #[weak]
        counter_lbl,
        move |_| {
            number.set(number.get() + 1);
            counter_lbl.set_label(&format!("You pressed the button {} times!", number.get()));
        }
    ));

    // ----- Using a custom button -----

    // create another label
    let another_lbl = Label::builder()
        .label("A custom button with an internal counter:")
        .margin_top(12)
        .margin_bottom(6)
        .build();

    // create custom button
    let custom_btn = CounterButton::with_label("0");

    // ----- Putting it all in a container -----

    let gtk_box = gtk::Box::builder()
        .orientation(gtk::Orientation::Vertical)
        .margin_top(12)
        .margin_bottom(12)
        .margin_start(12)
        .margin_end(12)
        .build();
    gtk_box.append(&counter_lbl);
    gtk_box.append(&btn);
    gtk_box.append(&another_lbl);
    gtk_box.append(&custom_btn);

    // create a window and set the title
    let window = ApplicationWindow::builder()
        .application(app)
        .title("My Gtk App")
        .child(&gtk_box)
        .build();

    // present the window
    window.present();
}
