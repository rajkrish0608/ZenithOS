#![no_std]

extern crate alloc;
use alloc::vec::Vec;

pub struct Window {
    pub id: u64,
    pub x: usize,
    pub y: usize,
    pub width: usize,
    pub height: usize,
    pub z_order: u32,
    pub opacity: f32, // 0.0 to 1.0
}

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

    pub fn draw_pixel(&mut self, x: usize, y: usize, color: u32) {
        if x >= self.width || y >= self.height { return; }
        let offset = (y * self.stride + x) * 4;
        unsafe {
            let ptr = (self.base_addr + offset) as *mut u32;
            *ptr = color;
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

pub struct Compositor {
    windows: Vec<Window>,
    fb: GopFrameBuffer,
}

impl Compositor {
    pub fn new(base_addr: usize, stride: usize, width: usize, height: usize) -> Self {
        Self {
            windows: Vec::new(),
            fb: GopFrameBuffer::new(base_addr, stride, width, height),
        }
    }

    pub fn create_window(&mut self, id: u64, x: usize, y: usize, width: usize, height: usize) {
        self.windows.push(Window {
            id, x, y, width, height,
            z_order: self.windows.len() as u32,
            opacity: 0.9, // Semi-transparent by default
        });
        self.compose();
    }

    pub fn compose(&mut self) {
        // In a real compositor, we'd redraw regions based on Z-order.
        // Mock: just log that we are composing.
        // "Rendering N windows..."
    }
}

pub struct BitmappedFont;
impl BitmappedFont {
    pub fn draw_string(&self, _fb: &mut GopFrameBuffer, _x: usize, _y: usize, _text: &str, _color: u32) {
        // Mock implementation
    }
}
