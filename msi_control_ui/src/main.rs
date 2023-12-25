use {
    std::sync::mpsc,
    ksni::TrayService
};

mod tray;
mod controllers;

use tray::MsiControlTray;


enum Message {
    Quit
}

fn main() {
    let (tx, rx) = mpsc::sync_channel::<Message>(2);

    let tray = MsiControlTray::new(tx).unwrap();
    let tray_srv = TrayService::new(tray);
    // let tray_handle = tray_srv.handle();
    tray_srv.spawn();

    loop {
        match rx.recv() {
            Ok(Message::Quit) => {
                break
            }
            _ => {}
        }
    }

}
