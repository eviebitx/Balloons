pub fn handle_syscall(id: usize, _arg1: usize, _arg2: usize, _arg3: usize) -> isize {
    match id {
        0 => {
            // sys_read
            0
        }
        1 => {
            // sys_write
            // For now, we could print to our v_console if we wanted, but we'll just return success.
            0
        }
        _ => {
            // Unknown syscall
            -1
        }
    }
}
