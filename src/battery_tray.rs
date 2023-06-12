use hyper_x_cloud_ii_wireless::Device;
use ksni::{Tray, MenuItem, menu::{StandardItem}, ToolTip, TrayService, Handle};

pub struct TrayHandler {
    handle: Handle<BatteryTray>,
}

impl TrayHandler {
    pub fn new(tray: BatteryTray) -> Self {
        let tray_service = TrayService::new(tray);
        let handle = tray_service.handle();
        tray_service.spawn();
        TrayHandler {
            handle,
        }
    }

    pub fn update(&self, device: &Device) {
        self.handle.update(|tray: &mut BatteryTray| { tray.update(device); })
    }

    pub fn set_status(&mut self, message: &str) {
        self.handle.update(|tray: &mut BatteryTray| { tray.set_status(message); })
    }

    pub fn clear_status(&mut self) {
        self.handle.update(|tray: &mut BatteryTray| { tray.clear_status(); })
    }
}

#[derive(Debug)]
pub struct BatteryTray {
    battery_level: u8,
    charging: Option<bool>,
    muted: Option<bool>,
    mic_connected: Option<bool>,
    status_message: Option<String>,
}

impl BatteryTray {
    pub fn new() -> Self {
        BatteryTray {
            battery_level: 0,
            charging: None,
            muted: None,
            mic_connected: None,
            status_message: Some("No device found".to_string()),
        }
    }

    pub fn update(&mut self, device: &Device) {
        self.battery_level = device.battery_level;
        self.charging = device.charging;
        self.muted = device.muted;
        self.mic_connected = device.mic_connected;
    }

    pub fn set_status(&mut self, message: &str) {
        self.status_message = Some(message.to_string());
    }

    pub fn clear_status(&mut self) {
        self.status_message = None;
    }
}

impl Tray for BatteryTray {
    fn icon_name(&self) -> String {
        "headset".into()
    }
    fn menu(&self) -> Vec<MenuItem<Self>> {
        vec![
            StandardItem {
                label: "Exit".into(),
                icon_name: "application-exit".into(),
                activate: Box::new(|_| std::process::exit(0)),
                ..Default::default()
            }
            .into(),
        ]
    }
    fn tool_tip(&self) -> ToolTip {
        let description = match &self.status_message {
            Some(m) => m.clone(),
            None => {
                let mut description = format!("Battery level: {}%", self.battery_level);
                if let Some(charging) = self.charging {
                    if charging {
                        description += "\nCharging";
                    } else {
                        description += "\nNot charging";
                    }
                }
                if let Some(muted) = self.muted {
                    if muted {
                        description += "\nMuted";
                    } else {
                        description += "\nNot muted";
                    }
                }
                if let Some(mic_connected) = self.mic_connected {
                    if mic_connected {
                        description += "\nMicrophone connected";
                    } else {
                        description += "\nMicrophone not connected";
                    }
                }
                description
            },
        };
        ToolTip {
            title: "HyperX Cloud II".to_string(),
            description: description,
            icon_name: "".into(),
            icon_pixmap: Vec::new(),
        }
    }
}