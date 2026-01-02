#![no_std]
#![no_main]

use core::panic::PanicInfo;

mod drivers;
mod v_console;
mod syscalls;
mod ai;

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}

#[unsafe(no_mangle)]
pub extern "C" fn _start() -> ! {
    v_console::init();
    println!("V-OS War-Rig initialized. BlackArch modules standing by. Boss, give me a target and let's tear it apart.");

    // Initialize things
    drivers::pci::scan_pci_bus();
    ai::mistrial::init();

    loop {}
}
