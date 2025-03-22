use hyper_x_cloud_ii_wireless::devices::DeviceState;
use ksni::{menu::StandardItem, Handle, MenuItem, ToolTip, Tray, TrayService};

pub struct TrayHandler {
    handle: Handle<StatusTray>,
}

impl TrayHandler {
    pub fn new(tray: StatusTray) -> Self {
        let tray_service = TrayService::new(tray);
        let handle = tray_service.handle();
        tray_service.spawn();
        TrayHandler { handle }
    }

    pub fn update(&self, device_state: &DeviceState) {
        let message = if device_state.connected.unwrap_or(false) {
            Some(device_state.to_string())
        } else {
            None
        };
        let name = device_state.device_name.clone();
        self.handle.update(|tray| {
            tray.message = message;
            tray.device_name = name;
        })
    }
}

pub struct StatusTray {
    device_name: Option<String>,
    message: Option<String>,
}

impl StatusTray {
    pub fn new() -> Self {
        StatusTray {
            device_name: None,
            message: None,
        }
    }
}

impl Tray for StatusTray {
    fn id(&self) -> String {
        env!("CARGO_PKG_NAME").into()
    }
    fn icon_name(&self) -> String {
        "audio-headset".into()
    }
    fn tool_tip(&self) -> ToolTip {
        println!("tool_tip");
        let description = if let Some(message) = self.message.clone() {
            message
        } else {
            "Headset is not connected".to_string()
        };
        ToolTip {
            title: self.device_name.clone().unwrap_or("Unknown".to_string()),
            description,
            icon_name: "audio-headset".into(),
            icon_pixmap: Vec::new(),
        }
    }
    fn menu(&self) -> Vec<MenuItem<Self>> {
        println!("menu");
        let message = if let Some(message) = &self.message {
            message
        } else {
            &"Headset is not connected".to_string()
        };

        let mut state_items: Vec<MenuItem<Self>> = message
            .lines()
            .map(|line| {
                StandardItem {
                    label: line.to_string(),
                    enabled: false,
                    ..Default::default()
                }
                .into()
            })
            .collect();
        let exit = StandardItem {
            label: "Exit".into(),
            icon_name: "application-exit".into(),
            activate: Box::new(|_| std::process::exit(0)),
            ..Default::default()
        };
        state_items.push(exit.into());
        state_items
    }
}
