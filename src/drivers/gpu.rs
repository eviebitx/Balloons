use bootloader_api::info::{FrameBuffer, FrameBufferInfo, PixelFormat};
use spin::Mutex;
use super::font;

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
                    // Our font data: bit 7 is left-most pixel (or right? usually left is MSB 0x80)
                    // Let's assume standard: MSB is left pixel.
                    // My manual font data has bits set.
                    // Loop 0..8. If I test (row >> (7 - col_i)), I get left-to-right.
                    // The previous code used `(row >> col_i) & 1` which is right-to-left if col_i is 0..8 (0 is LSB).
                    // Standard bitmap fonts usually store row 0 as top row.

                    // Let's use `(row >> (7 - col_i)) & 1` for standard MSB-left.
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

            // Draw Background (Deep Grey)
            self.draw_rect(0, 0, info.width, info.height, [20, 20, 20]);

            // Draw Logo Box (Red)
            let box_size = 100;
            self.draw_rect(center_x - box_size/2, center_y - box_size/2, box_size, box_size, [200, 0, 0]);

            // Draw Text
            let text = "V-OS WAR-RIG";
            // Centering text roughly (8px per char)
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
