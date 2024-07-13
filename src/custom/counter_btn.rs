use std::cell::Cell;
use gtk::glib;
use gtk::prelude::*;
use gtk::subclass::prelude::*;

#[derive(Default)]
pub struct CounterButton {
    count: Cell<usize>,
}

#[glib::object_subclass]
impl ObjectSubclass for CounterButton {
    const NAME: &'static str = "HelloGtkRsCounterButton"; // should consist of crate-name and object-name in order to avoid name collisions. use UpperCamelCase.
    type Type = super::CounterButton; // refers to the actual GObject that will be created afterwards.
    type ParentType = gtk::Button; // is the GObject we inherit
}

// Trait shared by all GObjects
impl ObjectImpl for CounterButton {
    fn constructed(&self) {
        self.parent_constructed();
        self.obj().set_label(&self.count.get().to_string());
    }
}

// Trait shared by all widgets
impl WidgetImpl for CounterButton {}

// Trait shared by all buttons
impl ButtonImpl for CounterButton {
    fn clicked(&self) {
        self.count.set(self.count.get() + 1);
        self.obj().set_label(&self.count.get().to_string());
    }
}
