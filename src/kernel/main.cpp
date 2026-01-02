#include "v_console.h"

// Stubs for PMM and IDT
void init_pmm() {
    v_print("[KERNEL] PMM initialized (stub).\n");
}

void init_idt() {
    v_print("[KERNEL] IDT initialized (stub).\n");
}

extern "C" void kmain() {
    // 1. Initialize PMM
    init_pmm();

    // 2. Initialize IDT
    init_idt();

    // 3. Initialize V-Console and Awakening
    v_init();

    // 4. V-Shell Loop
    char buffer[256];
    while (1) {
        v_print("V> ");
        v_read_line(buffer, sizeof(buffer));

        // Simple echo handling or command parsing could go here
        // For now, just echo is implied by v_read_line implementation
        // (which calls v_print_char), but let's confirm execution or do something.

        if (buffer[0] != '\0') {
             // If we wanted to process commands:
             // if (strcmp(buffer, "help") == 0) ...
             // Since we just need to "echo input" and provide a shell interface:
             // The v_read_line function already echoes the characters as they are typed.
             // We can just print a newline or "Command not found" stub.
        }
    }
}
