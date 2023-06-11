use std::time::Duration;

use hyper_x_cloud_ii_wireless::{Device, DeviceError};
mod battery_tray;
use battery_tray::TrayHandler;

fn pair_device() -> Device {
    loop {
        match Device::new() {
            Ok(device) => break device,
            Err(error) => {
                eprintln!("{error}");
            }
        };
        std::thread::sleep(std::time::Duration::from_secs(1));
    }
}

//TODO: status messages
//TODO: use mute state
//TODO: make trayHandler dynamic

fn handle_error(error: DeviceError, device: &mut Device, tray_handler: &TrayHandler) {
    match error {
        DeviceError::HidError(hidapi::HidError::HidApiError { message }) => {
            if message == "No such device" {
                eprintln!("No device found.");
                // handle.update(|tray: &mut BatteryTray| { tray.set_status_message("No device found"); });
                *device = pair_device();
            } else {
                eprintln!("{message}");
            }
        }
        DeviceError::NoDeviceFound() => {
            eprintln!("{}", DeviceError::NoDeviceFound());
            // handle.update(|tray: &mut BatteryTray| { tray.set_status_message("No device found"); });
        }
        DeviceError::HeadSetOff() => {
            eprintln!("{}", DeviceError::HeadSetOff());
            // handle.update(|tray: &mut BatteryTray| { tray.set_status_message(&DeviceError::HeadSetOff().to_string()); });
        }
        error => {
            eprintln!("{error}");
        }
    }
}

fn main() {
    let tray_handler = TrayHandler::new();
    let mut device = pair_device();
    tray_handler.update(&device);

    // Run loop
    loop {
        std::thread::sleep(std::time::Duration::from_secs(1));
        match device.update_battery_level() {
            Ok(_) => {
                tray_handler.update(&device);
            },
            Err(error) => {
                handle_error(error, &mut device, &tray_handler);
                continue;
            },
        };
        match device.wait_for_updates(Duration::from_secs(30)) {
            Ok(_) => tray_handler.update(&device),
            Err(error) => {
                handle_error(error, &mut device, &tray_handler);
                continue;
            }
        }
    }
}
