use std::{f32::consts::PI, fs::File, io::Read};

use sdl2::{audio::AudioQueue, keyboard::Keycode};

use super::{chip8_context::Chip8Context, font::FONTS};

pub const FONT_OFFSET: u8 = 0x050;
pub const ROM_OFFSET: usize = 0x200;

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
        rom.read(&mut self.context.memory[ROM_OFFSET..])
    }

    pub fn load_font(&mut self) {
        let mut index = FONT_OFFSET as usize;
        let flat_fonts = FONTS.as_flattened();
        self.context.memory[index..index + flat_fonts.len()].copy_from_slice(flat_fonts);
        for sprite in FONTS {
            for pixel in sprite {
                self.context.memory[index] = pixel;
                index += 1;
            }
        }
    }

    pub fn set_keydown(&mut self, keycode: Keycode) {
        let char = Chip8Emulator::get_char_hex(keycode);
        self.context.held_keys[char as usize] = true;
        self.context.input = Some(char);
    }

    pub fn set_keyup(&mut self, keycode: Keycode) {
        let char = Chip8Emulator::get_char_hex(keycode);
        self.context.held_keys[char as usize] = false;
    }

    fn get_char_hex(keycode: Keycode) -> u8 {
        match keycode {
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
            _ => 0x00,
        }
    }

    pub fn audio(&self, audio_queue: &AudioQueue<i16>) {
        if self.context.sound == 0 {
            audio_queue.clear();
            return;
        }

        let sample_rate = 44100.0;
        let duration_secs = 1.0;
        let num_samples = (sample_rate * duration_secs) as usize;
        let frequency = 440.0; // Frequency of the beep (A4 note)
        let amplitude = 30000.0; // Amplitude: keep it below 32767 for i16

        let mut beep: Vec<i16> = Vec::with_capacity(num_samples);
        for i in 0..num_samples {
            let time = i as f32 / sample_rate;
            // Calculate the sine wave sample for the given time
            let sample = (amplitude * (2.0 * PI * frequency * time).sin()) as i16;
            beep.push(sample);
        }

        let _ = audio_queue.queue_audio(&beep);
    }
}
