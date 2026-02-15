#![no_std]

pub struct GopFrameBuffer {
    base_addr: usize,
    stride: usize,
    width: usize,
    height: usize,
}

impl GopFrameBuffer {
    pub fn new(base_addr: usize, stride: usize, width: usize, height: usize) -> Self {
        Self {
            base_addr,
            stride,
            width,
            height,
        }
    }

    /// Draw a pixel at (x, y) with a specific color (ARGB format)
    /// Formula: Address = Base + (y * Stride + x) * 4
    pub fn draw_pixel(&mut self, x: usize, y: usize, color: u32) {
        if x >= self.width || y >= self.height {
            return;
        }

        let offset = (y * self.stride + x) * 4;
        let pixel_addr = (self.base_addr + offset) as *mut u32;

        unsafe {
            *pixel_addr = color;
        }
    }

    pub fn clear_screen(&mut self, color: u32) {
        for y in 0..self.height {
            for x in 0..self.width {
                self.draw_pixel(x, y, color);
            }
        }
    }
}

pub struct BitmappedFont;

impl BitmappedFont {
    /// Draws a character on the framebuffer.
    /// This is a mock implementation for the 8x8 bitmap font.
    pub fn draw_char(&self, buffer: &mut GopFrameBuffer, x: usize, y: usize, _c: char, color: u32) {
        // In a real implementation, we would look up the bitmap for `c`.
        // Here we just draw a 8x8 block for demonstration.
        for dy in 0..8 {
            for dx in 0..8 {
                buffer.draw_pixel(x + dx, y + dy, color);
            }
        }
    }

    pub fn draw_string(&self, buffer: &mut GopFrameBuffer, x: usize, y: usize, text: &str, color: u32) {
        let mut curr_x = x;
        for c in text.chars() {
            self.draw_char(buffer, curr_x, y, c, color);
            curr_x += 10; // Advance cursor
        }
    }
}
