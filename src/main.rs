use std::time::Duration;

mod status_tray;
use hyper_x_cloud_ii_wireless::devices::{connect_compatible_device, DeviceError};
use status_tray::{StatusTray, TrayHandler};

fn main() {
    let tray_handler = TrayHandler::new(StatusTray::new());
    loop {
        let mut device = loop {
            match connect_compatible_device() {
                Ok(d) => break d,
                Err(e) => println!("Connecting failed with error: {e}"),
            }
            std::thread::sleep(Duration::from_secs(1));
        };

        // Run loop
        loop {
            std::thread::sleep(Duration::from_secs(1));
            match device.refresh_state() {
                Ok(()) => (),
                Err(error) => {
                    eprintln!("{error}");
                    device.get_device_state_mut().connected = None;
                    tray_handler.update(device.get_device_state());
                    break; // try to reconnect
                }
            };
            tray_handler.update(device.get_device_state())
        }
    }
}
