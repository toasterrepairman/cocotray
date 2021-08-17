use tray_item::TrayItem;
use gtk;
use dialog_box::{warning, question};
use fsio::{directory, file, path};

fn main() {
    gtk::init().unwrap();

    let mut tray = TrayItem::new("smol", "hdsmol").unwrap();

    tray.add_label(":smol:").unwrap();

    tray.add_menu_item("Log", || {
        return;
    }).unwrap();

    tray.add_menu_item("Quit", || {
        println!("{}", warning("This will remove the tray icon.\nProceed?"));
        gtk::main_quit();
    }).unwrap();

    gtk::main();
}