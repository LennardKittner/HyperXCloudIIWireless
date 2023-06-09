use hyper_x_cloud_ii_wireless::Device;
use ksni::TrayService;
use battery_tray::BatteryTray;
mod battery_tray;

fn main() {
    let device = match Device::new() {
        Ok(device) => device,
        Err(error) => {
            eprintln!("{error}");
            std::process::exit(1);
        }
    };
    let (battery_level, charging) = match device.get_battery_level() {
        Ok(t) => t,
        Err(error) => {
            eprintln!("{error}");
            std::process::exit(1);
        }
    };
    println!("Battery level: {}%", battery_level);
    if charging {
        println!("Charging");
    } else {
        println!("Not charging");
    }

    let service = TrayService::new(BatteryTray::new(battery_level, charging));
    let handle = service.handle();
    service.spawn();

    // Run loop
    loop {
        std::thread::sleep(std::time::Duration::from_secs(1));
        let (battery_level, charging) = match device.get_battery_level() {
            Ok(t) => t,
            Err(error) => {
                eprintln!("{error}");
                continue;
            }
        };
        handle.update(|tray: &mut BatteryTray| {
            tray.update(battery_level, charging);
        });
    }
}
