use gtk::prelude::*;
use gtk::{Button, Label, Window, WindowType};

pub fn create_gui() {
    gtk::init().expect("Failed to initialize GTK.");

    let window = Window::new(WindowType::Toplevel);
    window.set_title("GTA 6 Download PC");
    window.set_default_size(300, 200);

    let label = Label::new(Some("Welcome to GTA 6 Download PC"));
    let button = Button::new_with_label("Download GTA 6");

    button.connect_clicked(|_| {
        // Trigger download logic here
    });

    let vbox = gtk::Box::new(gtk::Orientation::Vertical, 5);
    vbox.pack_start(&label, true, true, 0);
    vbox.pack_start(&button, true, true, 0);
    window.add(&vbox);

    window.show_all();
    window.connect_delete_event(|_, _| {
        gtk::main_quit();
        Inhibit(false)
    });

    gtk::main();
}