use std::time::Duration;

use hyper_x_cloud_ii_wireless::{Device, DeviceError};
mod battery_tray;
use battery_tray::{TrayHandler, BatteryTray};

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

fn handle_error(error: DeviceError, device: &mut Device, tray_handler: &mut TrayHandler) {
    match error {
        DeviceError::HidError(hidapi::HidError::HidApiError { message }) => {
            if message == "No such device" {
                eprintln!("No device found.");
                tray_handler.set_status("No device found.");
                *device = pair_device();
            } else {
                eprintln!("{message}");
            }
        }
        DeviceError::NoDeviceFound() => {
            eprintln!("{}", DeviceError::NoDeviceFound());
            device.clear_state();
            tray_handler.update(device);
            tray_handler.set_status( &DeviceError::NoDeviceFound().to_string());
        }
        DeviceError::HeadSetOff() => {
            eprintln!("{}", DeviceError::HeadSetOff());
            device.clear_state();
            tray_handler.update(device);
            tray_handler.set_status(&DeviceError::HeadSetOff().to_string());
        }
        error => {
            eprintln!("{error}");
        }
    }
}

fn main() {
    let mut tray_handler = TrayHandler::new(BatteryTray::new());
    let mut device = pair_device();
    tray_handler.update(&device);

    // Run loop
    loop {
        std::thread::sleep(std::time::Duration::from_secs(1));
        match device.update_battery_level() {
            Ok(_) => {
                tray_handler.clear_status();
                tray_handler.update(&device);
            },
            Err(error) => {
                handle_error(error, &mut device, &mut tray_handler);
                continue;
            },
        };
        match device.wait_for_updates(Duration::from_secs(60)) {
            Ok(_) => {
                tray_handler.clear_status();
                tray_handler.update(&device)
            },
            Err(DeviceError::NoResponse()) => (),
            Err(error) => {
                handle_error(error, &mut device, &mut tray_handler);
                continue;
            }
        }
    }
}
