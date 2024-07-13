use glib::Object;
use gtk::glib;

mod counter_btn;

glib::wrapper! {
    pub struct CounterButton(ObjectSubclass<counter_btn::CounterButton>)
        @extends gtk::Button, gtk::Widget,
        @implements gtk::Accessible, gtk::Actionable, gtk::Buildable, gtk::ConstraintTarget;
}

impl CounterButton {
    pub fn new() -> Self {
        Object::builder().build()
    }

    pub fn with_label(label: &str) -> Self {
        Object::builder().property("label", label).build()
    }
}
