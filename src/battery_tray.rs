use ksni::{Tray, MenuItem, menu::{StandardItem}};

#[derive(Debug)]
pub struct BatteryTray {
    battery_level: u8,
    charging: bool,
}

impl BatteryTray {
    pub fn new(battery_level: u8, charging: bool) -> Self {
        BatteryTray {
            battery_level,
            charging,
        }
    }

    pub fn refresh(&mut self, battery_level: u8, charging: bool) {
        self.battery_level = battery_level;
        self.charging = charging;
    }
}

impl Tray for BatteryTray {
    fn icon_name(&self) -> String {
        "headset".into()
    }
    fn title(&self) -> String {
        if self.charging {
            format!("Battery level: {}%\nCharging", self.battery_level).to_string()
        } else {
            format!("Battery level: {}%\nNot charging", self.battery_level).to_string()
        }
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
}