use x86_64::instructions::port::Port;

const CONFIG_ADDRESS: u16 = 0xCF8;
const CONFIG_DATA: u16 = 0xCFC;

pub fn scan_pci_bus() {
    // Basic PCI scan: Iterate over buses (0-255), devices (0-31), functions (0-7)
    for bus in 0..=255 {
        for device in 0..32 {
            if check_device(bus, device) {
                 // Found a device, checking functions...
                 // For now, we just acknowledge the device exists.
                 // We could print it if we had a console, but we'll assume the caller handles output or we add prints later.
            }
        }
    }
}

fn check_device(bus: u8, device: u8) -> bool {
    let vendor_id = read_pci_config_16(bus, device, 0, 0);
    if vendor_id == 0xFFFF {
        return false; // Device doesn't exist
    }
    true
}

fn read_pci_config_16(bus: u8, device: u8, func: u8, offset: u8) -> u16 {
    let address = (1 << 31)
        | ((bus as u32) << 16)
        | ((device as u32) << 11)
        | ((func as u32) << 8)
        | ((offset as u32) & 0xFC);

    let mut port_address: Port<u32> = Port::new(CONFIG_ADDRESS);
    let mut port_data: Port<u32> = Port::new(CONFIG_DATA);

    unsafe {
        port_address.write(address);
        let data = port_data.read();
        ((data >> ((offset & 2) * 8)) & 0xFFFF) as u16
    }
}
