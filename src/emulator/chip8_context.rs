use std::time::{Duration, Instant};

use sdl2::{pixels::Color, rect::Rect, render::Canvas, video::Window};

pub const LOOP_SPEED: f64 = 1.0 / 700.0;
pub const TIMER_SPEED: f64 = 1.0 / 60.0;
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
    pub sp: usize,

    // Special registers
    pub i: u16,
    pub pc: usize,
    pub delay: u8,
    pub sound: u8,
    last_timer_update: Instant,

    // Framebuffer
    pub frame_buffer: FrameBuffer,

    // Input
    pub held_keys: [bool; 16],
    pub input: Option<u8>,
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
            last_timer_update: Instant::now(),
            frame_buffer: FrameBuffer::new(),

            // Used to check for held keys
            held_keys: [false; 16],

            // Used when program is blocked on input
            input: None,
        }
    }

    pub fn get_next_instruction(&self) -> (u8, u8) {
        (self.memory[self.pc], self.memory[self.pc + 1])
    }

    pub fn increment_pc(&mut self) {
        self.pc += 2;
    }

    pub fn decrement_pc(&mut self) {
        self.pc -= 2;
    }

    pub fn stack_push(&mut self, value: u16) {
        self.stack[self.sp] = value;
        self.sp += 1;
    }

    pub fn stack_pop(&mut self) -> u16 {
        self.sp -= 1;
        self.stack[self.sp]
    }

    pub fn update_timers(&mut self) {
        let interval = Duration::from_secs_f64(TIMER_SPEED);
        let elapsed = self.last_timer_update.elapsed();

        if elapsed >= interval {
            if self.delay > 0 {
                self.delay -= 1;
            }

            if self.sound > 0 {
                self.sound -= 1;
            }

            self.last_timer_update = Instant::now();
        }
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
    dirty: bool,
}

impl FrameBuffer {
    pub fn new() -> Self {
        FrameBuffer {
            buffer: [false; WIDTH * HEIGHT],
            dirty: false,
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
        self.dirty = true;
    }
    pub fn is_dirty(&self) -> bool {
        self.dirty
    }
    pub fn clear(&mut self) {
        self.buffer = [false; WIDTH * HEIGHT];
        self.dirty = true;
    }
    pub fn render(&mut self, canvas: &mut Canvas<Window>) {
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
        self.dirty = false;
    }
}

impl Default for FrameBuffer {
    fn default() -> Self {
        Self::new()
    }
}
