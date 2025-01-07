use crate::{
    controllers::power_mode_controller::{PowerMode, PowerModeController},
    utils::capitalize_first,
    Message,
};
use ksni::{
    menu::{RadioGroup, RadioItem, StandardItem, SubMenu},
    Icon, Tray,
};
use std::sync::mpsc::SyncSender;

pub struct MsiControlTray {
    message_sender: SyncSender<Message>,
    current_power_mode: PowerMode,
    power_mode_controller: PowerModeController,
}

impl MsiControlTray {
    pub fn new(message_sender: SyncSender<Message>) -> Result<Self, dbus::Error> {
        let power_mode_controller = PowerModeController::new();
        let current_power_mode = power_mode_controller.current_power_mode()?;

        Ok(MsiControlTray {
            current_power_mode,
            message_sender,
            power_mode_controller,
        })
    }

    pub fn set_power_mode(&mut self, power_mode: PowerMode) -> Result<(), dbus::Error> {
        if self
            .power_mode_controller
            .set_power_mode(power_mode)
            .is_ok()
        {
            self.current_power_mode = power_mode;
            Ok(())
        } else {
            Err(dbus::Error::new_failed("Power mod setting was failed"))
        }
    }
}

impl Tray for MsiControlTray {
    fn id(&self) -> String {
        "msi_control".to_string()
    }

    fn activate(&mut self, _x: i32, _y: i32) {
        todo!("Have to do window app and send request to open/close it.");
    }

    fn icon_pixmap(&self) -> Vec<Icon> {
        vec![self.current_power_mode.icon()]
    }

    fn title(&self) -> String {
        let current_mode_str = capitalize_first(self.current_power_mode.to_string().as_str());
        format!("Current mode: {}", current_mode_str)
    }

    fn menu(&self) -> Vec<ksni::MenuItem<Self>> {
        let quit_tx = self.message_sender.clone();
        vec![
            // SubMenu {
            //     label: "Batary mode".into(),
            //     submenu: vec![],
            //     ..Default::default()
            // }
            // .into(),
            SubMenu {
                label: "Power mode".into(),
                submenu: vec![RadioGroup {
                    selected: self.current_power_mode.into(),
                    select: Box::new(move |tray: &mut MsiControlTray, idx| {
                        let selected_power_mode = idx.into();
                        tray.set_power_mode(selected_power_mode).unwrap();
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
                        },
                    ],
                }
                .into()],
                ..Default::default()
            }
            .into(),
            ksni::MenuItem::Separator,
            StandardItem {
                label: "Quit".to_string(),
                enabled: true,
                activate: Box::new(move |_| {
                    quit_tx.send(Message::Quit).unwrap();
                }),
                ..Default::default()
            }
            .into(),
        ]
    }
}
