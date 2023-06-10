use ksni::{Tray, MenuItem, menu::{StandardItem}, ToolTip};

#[derive(Debug)]
pub struct BatteryTray {
    battery_level: u8,
    charging: bool,
    device_found: bool,
}

impl BatteryTray {
    pub fn new() -> Self {
        BatteryTray {
            battery_level: 0,
            charging: false,
            device_found: false,
        }
    }

    pub fn update(&mut self, battery_level: u8, charging: bool) {
        self.device_found = true;
        self.battery_level = battery_level;
        self.charging = charging;
    }

    pub fn no_device_found(&mut self) {
        self.device_found = false;
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
        let description = 
            if !self.device_found {
                "No device found".to_string()
            } else if self.charging {
                format!("Battery level: {}%\nCharging", self.battery_level)
            } else {
                format!("Battery level: {}%\nNot charging", self.battery_level)
            };
        ToolTip {
            title: "HyperX Cloud II".to_string(),
            description: description,
            icon_name: "".into(),
            icon_pixmap: Vec::new(),
        }
    }
}