use hyper_x_cloud_ii_wireless::Device;
use ksni::{Tray, MenuItem, menu::{StandardItem}, ToolTip, TrayService, Handle};

pub struct TrayHandler {
    handle: Handle<BatteryTray>,
}

impl TrayHandler {
    pub fn new() -> Self {
        let tray_service = TrayService::new(BatteryTray::new());
        let handle = tray_service.handle();
        tray_service.spawn();
        TrayHandler {
            handle,
        }
    }

    pub fn update(&self, device: &Device) {
        self.handle.update(|tray: &mut BatteryTray| { tray.update(device); })
    }
}

#[derive(Debug)]
pub struct BatteryTray {
    battery_level: u8,
    charging: bool,
    status_message: Option<String>,
}

impl BatteryTray {
    pub fn new() -> Self {
        BatteryTray {
            battery_level: 0,
            charging: false,
            status_message: Some("No device found".to_string()),
        }
    }

    pub fn update(&mut self, device: &Device) {
        self.battery_level = device.battery_level;
        self.charging = device.charging;
    }

    pub fn set_status_message(&mut self, message: &str) {
        self.status_message = Some(message.to_string());
    }

    pub fn clear_status_message(&mut self) {
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
        let status =
            match &self.status_message {
                Some(m) => m,
                None => "",
            };
        let state =
            if self.charging {
                format!("Battery level: {}%\nCharging", self.battery_level)
            } else {
                format!("Battery level: {}%\nNot charging", self.battery_level)
            };
        ToolTip {
            title: "HyperX Cloud II".to_string(),
            description: format!("{}\n{}", status, state),
            icon_name: "".into(),
            icon_pixmap: Vec::new(),
        }
    }
}