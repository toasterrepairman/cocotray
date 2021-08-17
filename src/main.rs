use tray_item::TrayItem;
use gtk;

fn main() {

    gtk::init().unwrap();

    let mut tray = TrayItem::new("smol", "hdsmol").unwrap();

    tray.add_label(":smol:").unwrap();

    tray.add_menu_item("Hello", || {
        println!("Hello!");
    }).unwrap();

    tray.add_menu_item("Quit", || {
        gtk::main_quit();
    }).unwrap();

    gtk::main();
}