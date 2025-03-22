use std::time::Duration;

mod status_tray;
use status_tray::{StatusTray, TrayHandler};
use hyper_x_cloud_ii_wireless::devices::{connect_compatible_device, DeviceError};

fn handle_error(error: DeviceError) -> String {
    match error {
        DeviceError::HidError(hidapi::HidError::HidApiError { message }) => {
            if message == "No such device" {
                eprintln!("No device found.");
                "No such device".to_string()
            } else {
                eprintln!("{message}");
                message
            }
        }
        error => {
            eprintln!("{error}");
            error.to_string()
        }
    }
}

//TODO: error handling e.g. reconnect on "no such device" error
fn main() {
    let mut device = loop {
        match connect_compatible_device() {
            Ok(d) => break d,
            Err(e) => println!("Connecting failed with error: {e}"),
        }
        std::thread::sleep(Duration::from_secs(1));
    };

    let tray_handler = TrayHandler::new(StatusTray::new());

    // Run loop
    loop {
        std::thread::sleep(Duration::from_secs(1));
        match device.refresh_state() {
            Ok(()) => (),
            Err(error) => {
                eprintln!("{}", error);
            }
        };
        tray_handler.update(device.get_device_state())
    }
}
