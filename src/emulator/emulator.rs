use std::{fs::File, io::Read};

use sdl2::keyboard::Keycode;

use super::{chip8_context::Chip8Context, font::FONTS};

#[derive(Debug)]
pub struct Chip8Emulator {
    pub context: Chip8Context,
    pub mode: EmulatorMode,
    input_queue: Vec<u8>,
}

#[derive(Debug)]
pub enum EmulatorMode {
    Run,
    Step,
}

impl Chip8Emulator {
    pub fn new(mode: EmulatorMode) -> Chip8Emulator {
        let mut out = Chip8Emulator {
            context: Chip8Context::new(),
            mode,
            input_queue: vec![],
        };

        out.load_font();

        out
    }

    pub fn read_rom_into_memory(&mut self, mut rom: File) -> Result<usize, std::io::Error> {
        rom.read(&mut self.context.memory[0x200..])
    }

    pub fn load_font(&mut self) {
        let mut index = 0x50;
        let flat_fonts = FONTS.as_flattened();
        self.context.memory[0x50..0x50 + flat_fonts.len()].copy_from_slice(flat_fonts);
        for sprite in FONTS {
            for pixel in sprite {
                self.context.memory[index] = pixel;
                index += 1;
            }
        }
    }

    pub fn push_input(&mut self, keycode: Keycode) {
        let char = match keycode {
            Keycode::Num1 => 0x01,
            Keycode::Num2 => 0x02,
            Keycode::Num3 => 0x03,
            Keycode::Num4 => 0x04,
            Keycode::Num5 => 0x05,
            Keycode::Num6 => 0x06,
            Keycode::Num7 => 0x07,
            Keycode::Num8 => 0x08,
            Keycode::Num9 => 0x09,
            Keycode::Num0 => 0x00,
            Keycode::A => 0x0A,
            Keycode::B => 0x0B,
            Keycode::C => 0x0C,
            Keycode::D => 0x0D,
            Keycode::E => 0x0E,
            Keycode::F => 0x0F,
            _ => return,
        };
        self.context.push_input(char);
    }
}
