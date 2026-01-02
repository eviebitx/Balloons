use bootloader_api::info::{FrameBuffer, FrameBufferInfo, PixelFormat};
use spin::Mutex;
use super::font;
use core::fmt;

pub struct GpuDriver {
    framebuffer: Option<FrameBuffer>,
    info: Option<FrameBufferInfo>,
}

// Global GPU instance
pub static GPU: Mutex<GpuDriver> = Mutex::new(GpuDriver {
    framebuffer: None,
    info: None,
});

impl GpuDriver {
    pub fn init(&mut self, framebuffer: FrameBuffer) {
        self.info = Some(framebuffer.info());
        self.framebuffer = Some(framebuffer);
    }

    pub fn draw_pixel(&mut self, x: usize, y: usize, color: [u8; 3]) {
        if let (Some(fb), Some(info)) = (&mut self.framebuffer, self.info) {
            if x >= info.width || y >= info.height {
                return;
            }

            let byte_offset = (y * info.stride + x) * info.bytes_per_pixel;
            let buffer = fb.buffer_mut();

            match info.pixel_format {
                PixelFormat::Rgb => {
                    buffer[byte_offset] = color[0];     // R
                    buffer[byte_offset + 1] = color[1]; // G
                    buffer[byte_offset + 2] = color[2]; // B
                }
                PixelFormat::Bgr => {
                    buffer[byte_offset] = color[2];     // B
                    buffer[byte_offset + 1] = color[1]; // G
                    buffer[byte_offset + 2] = color[0]; // R
                }
                PixelFormat::U8 => {
                    let grayscale = (color[0] as u16 + color[1] as u16 + color[2] as u16) / 3;
                    buffer[byte_offset] = grayscale as u8;
                }
                _ => {}
            }
        }
    }

    pub fn draw_rect(&mut self, x: usize, y: usize, width: usize, height: usize, color: [u8; 3]) {
        for dy in 0..height {
            for dx in 0..width {
                self.draw_pixel(x + dx, y + dy, color);
            }
        }
    }

    pub fn draw_char(&mut self, x: usize, y: usize, c: char, color: [u8; 3]) {
        if let Some(raster) = font::get_raster(c) {
            for (row_i, row) in raster.iter().enumerate() {
                for col_i in 0..8 {
                    // Standard: MSB is left pixel.
                    if (row >> (7 - col_i)) & 1 == 1 {
                        self.draw_pixel(x + col_i, y + row_i, color);
                    }
                }
            }
        }
    }

    pub fn draw_string(&mut self, x: usize, y: usize, s: &str, color: [u8; 3]) {
        let mut cursor_x = x;
        for c in s.chars() {
            self.draw_char(cursor_x, y, c, color);
            cursor_x += 8;
        }
    }

    pub fn draw_boot_splash(&mut self) {
        if let Some(info) = self.info {
            let center_x = info.width / 2;
            let center_y = info.height / 2;

            self.draw_rect(0, 0, info.width, info.height, [20, 20, 20]);

            let box_size = 100;
            self.draw_rect(center_x - box_size/2, center_y - box_size/2, box_size, box_size, [200, 0, 0]);

            let text = "V-OS WAR-RIG";
            let text_width = text.len() * 8;
            self.draw_string(center_x - text_width / 2, center_y + 60, text, [255, 255, 255]);
        }
    }

    pub fn init_window_manager(&mut self) {
        if let Some(info) = self.info {
            let width = info.width;
            let height = info.height;
            let half_width = width / 2;

            // Left Side: V Personality (Dark Blueish)
            self.draw_rect(0, 0, half_width, height, [10, 10, 30]);
            self.draw_string(10, 10, "V-CHAT [ONLINE]", [0, 255, 0]);

            // Right Side: BlackArch Tools (Black)
            self.draw_rect(half_width, 0, half_width, height, [0, 0, 0]);
            self.draw_string(half_width + 10, 10, "BLACKARCH TOOLS [READY]", [255, 0, 0]);

            // Divider
            self.draw_rect(half_width - 1, 0, 2, height, [255, 255, 255]);
        }
    }
}

// Console Writer Implementation
pub struct ConsoleWriter {
    x_pos: usize,
    y_pos: usize,
    start_x: usize,
    start_y: usize,
    width: usize,
    height: usize,
    color: [u8; 3],
}

impl ConsoleWriter {
    pub const fn new(start_x: usize, start_y: usize, width: usize, height: usize, color: [u8; 3]) -> Self {
        Self {
            x_pos: start_x,
            y_pos: start_y,
            start_x,
            start_y,
            width,
            height,
            color,
        }
    }

    fn new_line(&mut self) {
        self.x_pos = self.start_x;
        self.y_pos += 8 + 2; // Line height + padding
        if self.y_pos >= self.start_y + self.height {
             // Reset to top (scroll not implemented)
             // Or better: wrap around or clear.
             // For now, clear area logic is complex without backing store.
             // Just wrap to top.
             self.y_pos = self.start_y;
             // Clear screen area?
             // Need access to GPU to clear.
             // Can't easily do it here without locking GPU again.
             // We'll just overwrite for now.
        }
    }

    pub fn write_byte(&mut self, byte: u8) {
        match byte {
             b'\n' => self.new_line(),
             0x08 => { // Backspace
                 if self.x_pos > self.start_x {
                     self.x_pos -= 8;
                     // Draw space to erase (requires GPU access)
                     let mut gpu = GPU.lock();
                     gpu.draw_char(self.x_pos, self.y_pos, ' ', [0,0,0]); // Assuming black background
                 }
             },
             byte => {
                 if self.x_pos >= self.start_x + self.width {
                     self.new_line();
                 }
                 let c = byte as char;
                 let mut gpu = GPU.lock();
                 gpu.draw_char(self.x_pos, self.y_pos, c, self.color);
                 self.x_pos += 8;
             }
        }
    }

    pub fn write_string(&mut self, s: &str) {
        for byte in s.bytes() {
            self.write_byte(byte);
        }
    }
}

impl fmt::Write for ConsoleWriter {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        self.write_string(s);
        Ok(())
    }
}

// Global Consoles
pub static LEFT_CONSOLE: Mutex<ConsoleWriter> = Mutex::new(ConsoleWriter::new(10, 40, 380, 500, [0, 255, 0])); // Green Text
pub static RIGHT_CONSOLE: Mutex<ConsoleWriter> = Mutex::new(ConsoleWriter::new(410, 40, 380, 500, [255, 255, 255])); // White Text

// Macros for GUI printing
#[macro_export]
macro_rules! left_print {
    ($($arg:tt)*) => ($crate::drivers::gpu::left_print_fmt(format_args!($($arg)*)));
}

#[macro_export]
macro_rules! left_println {
    () => ($crate::left_print!("\n"));
    ($($arg:tt)*) => ($crate::left_print!("{}\n", format_args!($($arg)*)));
}

#[macro_export]
macro_rules! right_print {
    ($($arg:tt)*) => ($crate::drivers::gpu::right_print_fmt(format_args!($($arg)*)));
}

#[macro_export]
macro_rules! right_println {
    () => ($crate::right_print!("\n"));
    ($($arg:tt)*) => ($crate::right_print!("{}\n", format_args!($($arg)*)));
}

pub fn left_print_fmt(args: fmt::Arguments) {
    use core::fmt::Write;
    LEFT_CONSOLE.lock().write_fmt(args).unwrap();
}

pub fn right_print_fmt(args: fmt::Arguments) {
    use core::fmt::Write;
    RIGHT_CONSOLE.lock().write_fmt(args).unwrap();
}
