#![no_std]
#![cfg_attr(not(test), no_main)]

// Allow std during testing for the test harness
#[cfg(test)]
#[macro_use]
extern crate std;

use core::sync::atomic::{AtomicBool, Ordering};

#[cfg(not(test))]
use core::panic::PanicInfo;

/// The "Heartbeat" of the AI Controller.
/// This module handles the global 'Pause' state from the Desktop Widget.
pub static AI_ACTIVE: AtomicBool = AtomicBool::new(true);

#[no_mangle]
#[cfg(not(test))] // Don't compile _start when testing
pub extern "C" fn _start() -> ! {
    // Initializing hardware, memory management, and GDT
    init_aegis_subsystems();

    loop {
        if AI_ACTIVE.load(Ordering::Relaxed) {
            // Execute AI-driven kernel tasks
            process_ai_agent_queue();
        } else {
            // AI is paused; only system-critical tasks run
            wait_for_interrupt();
        }
    }
}

fn init_aegis_subsystems() {
    // Setup for AI-centric memory pools
    // Integration with BlackArch-style toolchains
}

fn process_ai_agent_queue() {
    // Logic for AI-driven automated pentesting
    // This hooks into the sidebar terminal to report activity
}

fn wait_for_interrupt() {
    // Placeholder for waiting for interrupt
    // In x86 this would be `hlt` instruction or similar
}

#[panic_handler]
#[cfg(not(test))] // Don't use this panic handler when testing
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ai_active_toggle() {
        // Initial state
        assert!(AI_ACTIVE.load(Ordering::Relaxed));

        // Toggle off
        AI_ACTIVE.store(false, Ordering::Relaxed);
        assert!(!AI_ACTIVE.load(Ordering::Relaxed));

        // Toggle on
        AI_ACTIVE.store(true, Ordering::Relaxed);
        assert!(AI_ACTIVE.load(Ordering::Relaxed));
    }
}
