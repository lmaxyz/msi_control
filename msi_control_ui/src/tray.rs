use std::{sync::mpsc::SyncSender, io::Cursor};
use ksni::{Tray, Icon, menu::{StandardItem, SubMenu, RadioGroup, RadioItem}};
use crate::{
    Message,
    controllers::power_mode_controller::{PowerModeController, PowerMode}
};


struct TrayIcons {
    leaf: Icon,
    balance: Icon,
    sport: Icon
}

impl TrayIcons {
    fn new() -> Self {
        let leaf_icon = get_icon("icons/leaf.png");
        let balance_icon = get_icon("icons/balance.png");
        let sport_icon = get_icon("icons/performance.png");

        TrayIcons {
            leaf: leaf_icon,
            balance: balance_icon,
            sport: sport_icon
        }
    }

    fn get_power_mode_icon(&self, power_mode: PowerMode) -> Icon {
        match power_mode {
            PowerMode::Eco => self.leaf.clone(),
            PowerMode::Comfort => self.balance.clone(),
            PowerMode::Sport => self.sport.clone()
        }
    }
}


pub struct MsiControlTray {
    available_icons: TrayIcons,
    message_sender: SyncSender<Message>,
    current_power_mode: PowerMode,
    power_mode_controller: PowerModeController
}

impl MsiControlTray {
    pub fn new(message_sender: SyncSender<Message>) -> Result<Self, dbus::Error> {
        let available_icons = TrayIcons::new();
        let power_mode_controller = PowerModeController::new();
        let current_power_mode = power_mode_controller.get_current_power_mode()?;

        Ok(MsiControlTray {
            current_power_mode,
            available_icons,
            message_sender,
            power_mode_controller
        })
    }

    pub fn set_power_mode(&mut self, power_mode: PowerMode) -> Result<(), dbus::Error> {
        self.power_mode_controller.set_power_mode(power_mode)?;
        Ok(())
    }

    fn _get_current_power_mode(&self) -> Result<PowerMode, dbus::Error> {
        self.power_mode_controller.get_current_power_mode()
    }
}

impl Tray for MsiControlTray {
    fn icon_pixmap(&self) -> Vec<Icon> {
        vec![
            self.available_icons.get_power_mode_icon(self.current_power_mode)
        ]
    }
    
    fn title(&self) -> String {
        let current_mode_str = match self.current_power_mode {
            PowerMode::Eco => "Eco",
            PowerMode::Comfort => "Comfort",
            PowerMode::Sport => "Sport",
        };
        format!("Current mode: {}", current_mode_str)
    }

    fn menu(&self) -> Vec<ksni::MenuItem<Self>> {
        let quit_tx = self.message_sender.clone();
        vec![
            SubMenu{
                label: "Power mode".into(),
                submenu: vec![
                    RadioGroup {
                        selected: self.current_power_mode.into(),
                        select: Box::new(move |tray: &mut MsiControlTray, j| {
                            let selected_power_mode = j.into();
                            if tray.set_power_mode(selected_power_mode).is_ok() {
                                tray.current_power_mode = selected_power_mode;
                            };
                        }),
                        options: vec![
                            RadioItem {
                                label: "Eco".into(),
                                ..Default::default()
                            },
                            RadioItem {
                                label: "Comfort".into(),
                                ..Default::default()
                            },
                            RadioItem {
                                label: "Sport".into(),
                                ..Default::default()
                            }
                        ]
                    }.into(),
                ],
                ..Default::default()
            }.into(),
            
            ksni::MenuItem::Separator,

            StandardItem {
                label: "Quit".to_string(),
                enabled: true,
                activate: Box::new(move |_| {
                    quit_tx.send(Message::Quit).unwrap();
                }),
                ..Default::default()
            }.into()
        ]
    }
}

fn get_icon(path: &str) -> Icon {
    let bytes = std::fs::read(path).unwrap();
    let icon_cursor = Cursor::new(bytes);
    let icon_decoder = png::Decoder::new(icon_cursor);
    let mut icon_reader = icon_decoder.read_info().unwrap();
    let mut icon_buf = vec![0; icon_reader.info().raw_bytes()];
    icon_reader.next_frame(&mut icon_buf).unwrap();
    Icon{data: icon_buf, height: 32, width: 32}
}
