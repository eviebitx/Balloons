use crate::drivers::keyboard;
use crate::{right_print, right_println, left_println, serial_println};
use crate::kernel::mission_control;
use crate::drivers::pci;

// Simple command buffer
const BUFFER_SIZE: usize = 128;
static mut INPUT_BUFFER: [u8; BUFFER_SIZE] = [0; BUFFER_SIZE];
static mut INPUT_LEN: usize = 0;

pub fn run_loop() -> ! {
    right_println!("V-OS Terminal v0.1");
    right_println!("Type 'help' for commands.");
    right_print!("root@v-os:~$ ");

    loop {
        // Poll keyboard
        keyboard::poll();

        if let Some(c) = keyboard::pop_key() {
            match c {
                '\n' => {
                    right_println!();
                    unsafe {
                        if INPUT_LEN > 0 {
                            if let Ok(cmd_str) = core::str::from_utf8(&INPUT_BUFFER[..INPUT_LEN]) {
                                process_command(cmd_str);
                            }
                            INPUT_LEN = 0;
                        }
                    }
                    right_print!("root@v-os:~$ ");
                }
                '\x08' => { // Backspace
                    unsafe {
                        if INPUT_LEN > 0 {
                            INPUT_LEN -= 1;
                            // Send backspace to console (ConsoleWriter handles visual backspace)
                            right_print!("\x08");
                        }
                    }
                }
                c => {
                    unsafe {
                        if INPUT_LEN < BUFFER_SIZE {
                            INPUT_BUFFER[INPUT_LEN] = c as u8;
                            INPUT_LEN += 1;
                            right_print!("{}", c);
                        }
                    }
                }
            }
        }

        // Background tasks
        mission_control::tick();

        for _ in 0..1000 {
            core::hint::spin_loop();
        }
    }
}

fn process_command(cmd: &str) {
    let cmd = cmd.trim();
    match cmd {
        "help" => {
            right_println!("Available Commands:");
            right_println!("  help    - Show this message");
            right_println!("  scan    - Force PCI Bus Scan");
            right_println!("  status  - Show System Status");
            right_println!("  analyze - Run AI Analysis Simulation");
            right_println!("  panic   - Trigger Kernel Panic");
        }
        "scan" => {
            right_println!("[CMD] Initiating PCI Scan...");
            pci::scan_pci_bus();
            right_println!("[CMD] Scan Complete.");
        }
        "status" => {
            right_println!("[STATUS] V-OS War-Rig Active");
            right_println!("[STATUS] Mission Control: Running");
            right_println!("[STATUS] Drivers: Loaded");
        }
        "analyze" => {
            right_println!("[CMD] Requesting AI Analysis...");
            mission_control::analyze_target("Manual Input Data");
        }
        "panic" => {
            panic!("User requested panic!");
        }
        "" => {}
        _ => {
            right_println!("Unknown command: '{}'", cmd);
        }
    }
}
