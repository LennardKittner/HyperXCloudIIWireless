use hidapi::{HidApi, DeviceInfo};

const VENDOR_IDS: [u16; 2] = [0x0951, 0x03F0];
// Possible Cloud II Wireless product IDs
const PRODUCT_IDS: [u16; 2] = [0x1718, 0x018B];

const BATTERY_PACKET_4: [u8; 58] = [0x1b, 0x00, 0xa0, 0xc5, 0x4f, 0x91, 0x8e, 0xd1, 0xff, 0xff, 0x00, 0x00, 0x00, 0x00, 0x09, 0x00, 0x01, 0x02, 0x00, 0x01, 0x00, 0x84, 0x01, 0x1f, 0x00, 0x00, 0x00, 0x21, 0xbb, 0x0c, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x0];
const BATTERY_PACKET_1: [u8; 62] = [6, 0, 2, 0, 154, 0, 0, 104, 74, 142, 10, 0, 0, 0, 187, 2, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
const BATTERY_PACKET_2: [u8; 20] = {
    let mut packet = [0; 20];
    (packet[0], packet[1], packet[2], packet[3]) = (0x06, 0xff, 0xbb, 0x02);
    packet
};
const BATTERY_PACKET_3: [u8; 62] = {
    let mut packet = [0; 62];
    (packet[0], packet[1], packet[2], packet[3], packet[4], packet[5], packet[6], packet[7], packet[8], packet[9], packet[10], packet[11], packet[12], packet[13], packet[14], packet[15]) =
    (0x06, 0x00, 0x02, 0x00, 0x9a, 0x00, 0x00, 0x68, 0x4a, 0x8e, 0x0a, 0x00, 0x00, 0x00, 0xbb, 0x03);
    packet
};


const PACKETS: [&[u8]; 4] = [&BATTERY_PACKET_1, &BATTERY_PACKET_2, &BATTERY_PACKET_3, &BATTERY_PACKET_4];

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
        let mut response_buffer = [0u8; 20];
        println!("  packet: {:?}", PACKETS[i]);
        let _ = device.write(packet).map_err(|err| println!("{err}"));
        let len = device.read_timeout(&mut response_buffer, 1000).map_err(|err| println!("{err}")).unwrap();
        println!("  response: {:?}\n", &response_buffer[..len]);
    }
}