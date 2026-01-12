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
mod shell;

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    serial_print!("{}", info);
    if let Some(mut writer) = drivers::gpu::LEFT_CONSOLE.try_lock() {
        use core::fmt::Write;
        let _ = writeln!(writer, "PANIC: {}", info);
    }
    loop {}
}

entry_point!(kernel_main);

fn kernel_main(boot_info: &'static mut BootInfo) -> ! {
    v_console::init();

    // No interrupts on stable yet, using polling in the shell loop.

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

    // Initialize drivers
    drivers::pci::scan_pci_bus();
    ai::mistrial::init();

    // Check RAM (Simple report)
    // boot_info.memory_regions is a MemoryRegionList, which derefs to slice.
    let regions = &boot_info.memory_regions;
    let mut usable_ram = 0;
    for region in regions.iter() {
         if region.kind == bootloader_api::info::MemoryRegionKind::Usable {
             usable_ram += region.end - region.start;
         }
    }
    left_println!(" [INIT] RAM Checked: {} MB Usable", usable_ram / 1024 / 1024);

    left_println!(" [INIT] I'm feeling smart today, Boss.");

    // Initialize Mission Control
    kernel::mission_control::init();

    // Enter interactive shell (Desktop)
    shell::run_loop();
}
