use {ksni::TrayService, std::sync::mpsc};

mod controllers;
mod tray;
mod utils;

use tray::MsiControlTray;

enum Message {
    Quit,
}

fn main() {
    let (tx, rx) = mpsc::sync_channel::<Message>(2);

    let tray = MsiControlTray::new(tx).unwrap();
    let tray_srv = TrayService::new(tray);
    tray_srv.spawn();

    loop {
        match rx.recv() {
            Ok(Message::Quit) => break,
            _ => {}
        }
    }
}
