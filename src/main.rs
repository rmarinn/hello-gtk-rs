use gtk::prelude::*;
use gtk::{glib, glib::clone, Application, ApplicationWindow, Button, Label};
use std::cell::Cell;
// use std::rc::Rc;

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
    // create a button
    let counter_lbl = Label::builder()
        .label("You pressed the button 0 times!")
        .build();

    // create a button
    let btn = Button::builder()
        .label("Click me!")
        .margin_top(12)
        .build();

    // reference-counted object with inner-mutability
    // let number = Rc::new(Cell::new(0));
    let number = Cell::new(0);

    // connect callbacks
    btn.connect_clicked(clone!(
        #[weak]
        counter_lbl,
        move |_| {
            number.set(number.get() + 1);
            counter_lbl.set_label(&format!("You pressed the button {} times!", number.get()));
        }
    ));

    let gtk_box = gtk::Box::builder()
        .orientation(gtk::Orientation::Vertical)
        .margin_top(12)
        .margin_bottom(12)
        .margin_start(12)
        .margin_end(12)
        .build();
    gtk_box.append(&counter_lbl);
    gtk_box.append(&btn);

    // create a window and set the title
    let window = ApplicationWindow::builder()
        .application(app)
        .title("My Gtk App")
        .child(&gtk_box)
        .build();

    // present the window
    window.present();
}
