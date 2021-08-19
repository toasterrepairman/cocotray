use tray_item::TrayItem;
use gtk;
use dialog_box::{warning, information, pick_number};
use fsio::{directory, file, path};

fn main() {
    gtk::init().unwrap();

    let mut tray = TrayItem::new("smol", "hdsmol").unwrap();

    tray.add_label(":smol:").unwrap();

    tray.add_menu_item("Burgerfy Temp", || {
        // makes Celsius into Fah
        let normal_units = pick_number("Temp in Celsius")
            .trim()
            .parse::<f64>()
            .expect("Could not convert value to number.");
        let burger_units = normal_units * 1.8 + 32.0;
        information(&burger_units.to_string());
    }).unwrap();

    tray.add_menu_item("Quit", || {
        println!("{}", warning("This will remove the tray icon.\nProceed?"));
        gtk::main_quit();
    }).unwrap();

    gtk::main();
}