use crate::drivers::keyboard;
use crate::{right_print, right_println, left_println};

pub fn run_loop() -> ! {
    right_println!("V-OS Terminal v0.1");
    right_print!("root@v-os:~$ ");

    loop {
        // Poll keyboard
        keyboard::poll();

        if let Some(c) = keyboard::pop_key() {
            match c {
                '\n' => {
                    right_println!();
                    process_command();
                    right_print!("root@v-os:~$ ");
                }
                c => {
                    right_print!("{}", c);
                    // Add to command buffer (omitted for brevity)
                }
            }
        }

        // Cpu relaxation?
        // With polling, we shouldn't halt aggressively or we miss keys.
        // Or we use a small delay.
        for _ in 0..1000 {
            core::hint::spin_loop();
        }

        // Also tick mission control here?
        // kernel::mission_control::tick();
    }
}

fn process_command() {
    // Mock command processing
    left_println!("[CMD] Command received.");
}
