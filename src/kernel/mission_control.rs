use crate::serial_println;
use crate::ai::mistrial;
use crate::left_println;

pub fn init() {
    left_println!("Mission Control online. Scanning for opportunities, Boss.");
    serial_println!("[MISSION_CONTROL] System initialized. Scheduler active.");
}

pub fn analyze_target(data: &str) {
    serial_println!("[MISSION_CONTROL] Sending data to V-Intelligence...");
    mistrial::analyze(data);
}

pub fn autonomous_loop() -> ! {
    let mut ticks: u64 = 0;

    loop {
        ticks = ticks.wrapping_add(1);

        // Simulate "scanning" every few million cycles (busy wait)
        if ticks % 1_000_000 == 0 {
            if ticks % 10_000_000 == 0 {
                left_println!(" [SCANNER] Signal detected on port 8080...");
                analyze_target("HTTP Header: Server: Vulnerable-IIS/6.0");

                execute_mission("exploit_stage1.bin");
            }
        }

        x86_64::instructions::hlt();
    }
}

fn execute_mission(mission_name: &str) {
    left_println!(" [JOB] Executing mission: {}", mission_name);
    serial_println!("[MISSION_CONTROL] Spawning task for: {}", mission_name);

    for _ in 0..1000 {
        core::hint::spin_loop();
    }

    left_println!(" [JOB] Mission '{}' completed successfully.", mission_name);
}
