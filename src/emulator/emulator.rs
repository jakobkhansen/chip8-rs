use std::{fs::File, io::Read};

use super::{chip8_context::Chip8Context, font::FONTS};

#[derive(Debug)]
pub struct Chip8Emulator {
    pub context: Chip8Context,
    pub mode: EmulatorMode,
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
}
