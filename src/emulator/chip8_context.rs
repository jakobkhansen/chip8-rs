use sdl2::{pixels::Color, rect::Rect, render::Canvas, video::Window};

use super::font::FONTS;

pub const WIDTH: usize = 64;
pub const HEIGHT: usize = 32;
pub const SCALE: u32 = 10;

#[derive(Debug)]
pub struct Chip8Context {
    // RAM
    pub memory: [u8; 4096],

    //  Registers
    pub v: [u8; 16],

    // Stack and stack-pointer
    pub stack: [u16; 16],
    pub sp: u8,

    // Special registers
    pub i: u16,
    pub pc: usize,
    pub delay: u8,
    pub sound: u8,

    // Framebuffer
    pub frame_buffer: FrameBuffer,
}

impl Chip8Context {
    pub fn new() -> Self {
        Chip8Context {
            memory: [0; 4096],
            v: [0; 16],
            stack: [0; 16],
            sp: 0,
            i: 0,
            pc: 0x200,
            delay: 0,
            sound: 0,
            frame_buffer: FrameBuffer::new(),
        }
    }

    pub fn get_next_instruction(&self) -> (u8, u8) {
        (self.memory[self.pc], self.memory[self.pc + 1])
    }

    pub fn increment_pc(&mut self) {
        self.pc += 2;
    }
}

impl Default for Chip8Context {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug)]
pub struct FrameBuffer {
    buffer: [bool; WIDTH * HEIGHT],
}

impl FrameBuffer {
    pub fn new() -> Self {
        FrameBuffer {
            buffer: [false; WIDTH * HEIGHT],
        }
    }
    pub fn get_pixel(&self, x: usize, y: usize) -> Option<bool> {
        let index = x * HEIGHT + y;
        self.buffer.get(index).copied()
    }
    pub fn set_pixel(&mut self, x: usize, y: usize, value: bool) {
        let index = x * HEIGHT + y;
        if let Some(elem) = self.buffer.get_mut(index) {
            *elem = value;
        }
    }
    pub fn clear(&mut self) {
        self.buffer = [false; WIDTH * HEIGHT];
    }
    pub fn render(&self, canvas: &mut Canvas<Window>) {
        canvas.set_draw_color(Color::BLACK);
        canvas.clear();
        canvas.set_draw_color(Color::WHITE);

        for y in 0..HEIGHT {
            for x in 0..WIDTH {
                if self.get_pixel(x, y).expect("Invalid index") {
                    let rect = Rect::new(
                        (x as u32 * SCALE) as i32,
                        (y as u32 * SCALE) as i32,
                        SCALE,
                        SCALE,
                    );
                    let _ = canvas.fill_rect(rect);
                }
            }
        }
        canvas.present();
    }
}

impl Default for FrameBuffer {
    fn default() -> Self {
        Self::new()
    }
}
