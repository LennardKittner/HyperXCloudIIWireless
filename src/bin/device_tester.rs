use hidapi::{HidApi, DeviceInfo};

const VENDOR_IDS: [u16; 2] = [0x0951, 0x03F0];
// Possible Cloud II Wireless product IDs
const PRODUCT_IDS: [u16; 2] = [0x1718, 0x018B];

const BATTERY_PACKET: [u8; 62] = [6, 0, 2, 0, 154, 0, 0, 104, 74, 142, 10, 0, 0, 0, 187, 2, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
const BATTERY_PACKET_2: [u8; 20] = [6, 0, 2, 0, 154, 0, 0, 104, 74, 142, 10, 0, 0, 0, 187, 2, 0, 0, 0, 0];
const BATTERY_PACKET_3: [u8; 20] = {
    let mut packet = [0; 20];
    (packet[0], packet[1], packet[2], packet[3]) = (0x06, 0xff, 0xbb, 0x02);
    packet
};
const BATTERY_PACKET_4: [u8; 20] = {
    let mut packet = [0; 20];
    (packet[0], packet[1], packet[2], packet[3]) = (0x06, 0xff, 0xbb, 0x06);
    packet
};

const PACKETS: [&[u8]; 4] = [&BATTERY_PACKET, &BATTERY_PACKET_2, &BATTERY_PACKET_3, &BATTERY_PACKET_4];

fn main() {
    let hidapi = HidApi::new().unwrap();
    for device in hidapi.device_list() {
        if VENDOR_IDS.contains(&device.vendor_id()) && PRODUCT_IDS.contains(&device.product_id()) {
            test_device(device);
        }
    }
}

fn test_device(device_info: &DeviceInfo) {
    println!("Testing device: {}:{}:{}", device_info.vendor_id(), device_info.product_id(), device_info.interface_number());
    let hidapi = HidApi::new().unwrap();
    let device = device_info.open_device(&hidapi).unwrap();
    for (i, packet) in PACKETS.iter().enumerate() {
        let mut response_buffer = [0u8; 8];
        println!("packet: {}", i);
        let _ = device.write(packet).map_err(|err| println!("{err}"));
        let len = device.read_timeout(&mut response_buffer, 1000).map_err(|err| println!("{err}")).unwrap();
        println!("{:?}", &response_buffer[..len]);
    }
}