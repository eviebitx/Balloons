#![no_std]
#![no_main]

use core::panic::PanicInfo;
use bootloader_api::{BootInfo, entry_point};
use bootloader_api::info::Optional;

mod drivers;
mod v_console;
mod syscalls;
mod ai;
mod kernel;

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    serial_print!("{}", info);
    println!("{}", info);
    loop {}
}

entry_point!(kernel_main);

fn kernel_main(boot_info: &'static mut BootInfo) -> ! {
    v_console::init();
    println!("V-OS War-Rig initialized. BlackArch modules standing by. Boss, give me a target and let's tear it apart.");

    // Initialize GPU if available
    if let Optional::Some(framebuffer) = core::mem::replace(&mut boot_info.framebuffer, Optional::None) {
        let mut gpu = drivers::gpu::GPU.lock();
        gpu.init(framebuffer);
        gpu.draw_boot_splash();

        // Short delay to admire the splash (simulated)
        for _ in 0..1000000 { core::hint::spin_loop(); }

        gpu.init_window_manager();
    } else {
         println!(" [WARNING] No Framebuffer found. Falling back to Text Mode.");
    }

    // Initialize things
    drivers::pci::scan_pci_bus();
    ai::mistrial::init();

    // Handover to Mission Control
    kernel::mission_control::init();
    kernel::mission_control::autonomous_loop();
}
