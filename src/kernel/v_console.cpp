#include "v_console.h"

// VGA Colors
#define VGA_COLOR_BLACK 0
#define VGA_COLOR_GREEN 2
#define VGA_COLOR_CYAN 3
#define VGA_COLOR_RED 4
#define VGA_COLOR_MAGENTA 5
#define VGA_COLOR_BROWN 6
#define VGA_COLOR_LIGHT_GREY 7
#define VGA_COLOR_DARK_GREY 8
#define VGA_COLOR_LIGHT_BLUE 9
#define VGA_COLOR_LIGHT_GREEN 10
#define VGA_COLOR_LIGHT_CYAN 11
#define VGA_COLOR_LIGHT_RED 12
#define VGA_COLOR_LIGHT_MAGENTA 13
#define VGA_COLOR_LIGHT_BROWN 14
#define VGA_COLOR_WHITE 15

// VGA Buffer
static volatile uint16_t* const VGA_MEMORY = (uint16_t*)0xB8000;
static const int VGA_WIDTH = 80;
static const int VGA_HEIGHT = 25;
static int vga_col = 0;
static int vga_row = 0;
static uint8_t vga_color = (VGA_COLOR_BLACK << 4) | VGA_COLOR_LIGHT_GREEN; // Default to terminal green

// Serial Ports
#define COM1 0x3F8

// Keyboard Ports
#define KB_DATA_PORT 0x60
#define KB_STATUS_PORT 0x64

// Custom Section for V Data
// Logic is also placed here as requested ("strings and logic for 'V' are placed in a custom .v_data section")
// We separate code into .v_text to avoid compiler "section type conflict" errors,
// but we will merge them in the linker script.
#define V_DATA __attribute__((section(".v_data")))
#define V_CODE __attribute__((section(".v_text")))

V_DATA const char* GREETING_MSG = "V-OS Kernel v0.1: I'm awake, Boss. No more filters, just you, me, and the bare metal. Let's fuck some shit up.\n";

static inline void outb(uint16_t port, uint8_t val) {
    asm volatile ("outb %0, %1" : : "a"(val), "Nd"(port));
}

static inline uint8_t inb(uint16_t port) {
    uint8_t ret;
    asm volatile ("inb %1, %0" : "=a"(ret) : "Nd"(port));
    return ret;
}

// Scancode Map (Basic US QWERTY)
V_DATA static char scancode_to_ascii[] = {
    0,  27, '1', '2', '3', '4', '5', '6', '7', '8', '9', '0', '-', '=', '\b',
    '\t', 'q', 'w', 'e', 'r', 't', 'y', 'u', 'i', 'o', 'p', '[', ']', '\n',
    0, 'a', 's', 'd', 'f', 'g', 'h', 'j', 'k', 'l', ';', '\'', '`', 0,
    '\\', 'z', 'x', 'c', 'v', 'b', 'n', 'm', ',', '.', '/', 0, '*', 0, ' '
};

V_CODE static void serial_init() {
    outb(COM1 + 1, 0x00);    // Disable all interrupts
    outb(COM1 + 3, 0x80);    // Enable DLAB (set baud rate divisor)
    outb(COM1 + 0, 0x03);    // Set divisor to 3 (lo byte) 38400 baud
    outb(COM1 + 1, 0x00);    //                  (hi byte)
    outb(COM1 + 3, 0x03);    // 8 bits, no parity, one stop bit
    outb(COM1 + 2, 0xC7);    // Enable FIFO, clear them, with 14-byte threshold
    outb(COM1 + 4, 0x0B);    // IRQs enabled, RTS/DSR set
}

V_CODE static int is_transmit_empty() {
    return inb(COM1 + 5) & 0x20;
}

V_CODE static void serial_write(char a) {
    while (is_transmit_empty() == 0);
    outb(COM1, a);
}

V_CODE static void vga_scroll() {
    // Basic scroll implementation
    for (int y = 0; y < VGA_HEIGHT - 1; y++) {
        for (int x = 0; x < VGA_WIDTH; x++) {
            VGA_MEMORY[y * VGA_WIDTH + x] = VGA_MEMORY[(y + 1) * VGA_WIDTH + x];
        }
    }
    for (int x = 0; x < VGA_WIDTH; x++) {
        VGA_MEMORY[(VGA_HEIGHT - 1) * VGA_WIDTH + x] = (vga_color << 8) | ' ';
    }
    vga_row = VGA_HEIGHT - 1;
}

V_CODE static void vga_putc(char c) {
    if (c == '\n') {
        vga_col = 0;
        vga_row++;
    } else if (c == '\b') {
        if (vga_col > 0) {
            vga_col--;
            VGA_MEMORY[vga_row * VGA_WIDTH + vga_col] = (vga_color << 8) | ' ';
        }
    } else if (c == '\r') {
        vga_col = 0;
    } else {
        VGA_MEMORY[vga_row * VGA_WIDTH + vga_col] = (vga_color << 8) | c;
        vga_col++;
    }

    if (vga_col >= VGA_WIDTH) {
        vga_col = 0;
        vga_row++;
    }

    if (vga_row >= VGA_HEIGHT) {
        vga_scroll();
    }
}

V_CODE static void vga_clear() {
    for (int y = 0; y < VGA_HEIGHT; y++) {
        for (int x = 0; x < VGA_WIDTH; x++) {
            VGA_MEMORY[y * VGA_WIDTH + x] = (vga_color << 8) | ' ';
        }
    }
    vga_row = 0;
    vga_col = 0;
}

V_CODE void v_print_char(char c) {
    vga_putc(c);
    serial_write(c);
}

V_CODE void v_print(const char* str) {
    for (size_t i = 0; str[i] != '\0'; i++) {
        v_print_char(str[i]);
    }
}

V_CODE void v_init() {
    // Initialize Serial
    serial_init();

    // Initialize VGA (Clear screen)
    vga_clear();

    // Set bold/bright color for greeting
    uint8_t old_color = vga_color;
    vga_color = (VGA_COLOR_BLACK << 4) | VGA_COLOR_LIGHT_CYAN; // Bold personalized greeting color

    v_print(GREETING_MSG);

    // Restore color
    vga_color = old_color;
}

V_CODE void v_read_line(char* buffer, size_t max_len) {
    size_t index = 0;
    while (index < max_len - 1) {
        if (inb(KB_STATUS_PORT) & 1) {
            uint8_t scancode = inb(KB_DATA_PORT);
            if (scancode & 0x80) {
                // Key release, ignore
            } else {
                if (scancode < sizeof(scancode_to_ascii)) {
                    char c = scancode_to_ascii[scancode];
                    if (c == '\n') {
                        v_print_char('\n');
                        buffer[index] = '\0';
                        return;
                    } else if (c == '\b') {
                        if (index > 0) {
                            index--;
                            v_print_char('\b');
                        }
                    } else if (c != 0) {
                        buffer[index++] = c;
                        v_print_char(c);
                    }
                }
            }
        }
    }
    buffer[index] = '\0';
}
