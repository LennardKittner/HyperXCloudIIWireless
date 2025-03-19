use hyper_x_cloud_ii_wireless::devices::{cloud_ii_wireless_dts::CloudIIWirelessDTS, Device};

fn main() {
    let mut device = match CloudIIWirelessDTS::new() {
        Ok(device) => device,
        Err(error) => {
            eprintln!("{error}");
            std::process::exit(1);
        }
    };

    if let Err(error) = device.refresh_state() {
        eprintln!("{error}");
        std::process::exit(1);
    };
    println!("Device State: {}", device.get_device_state());
}

#[test]
fn test_basic_device_access() {
    let _ = match CloudIIWirelessDTS::new() {
        Ok(device) => device,
        Err(_) => return,
    };
}
