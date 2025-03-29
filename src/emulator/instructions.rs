use std::u16::MAX;

use crate::emulator::chip8_context::{HEIGHT, WIDTH};

use super::emulator::{Chip8Emulator, FONT_OFFSET};

impl Chip8Emulator {
    pub fn execute_instruction(&mut self) {
        let (start, end) = self.context.get_next_instruction();
        let full = ((start as u16) << 8) | (end as u16);

        let nibble_1 = (full >> 12) & 0xF; // First nibble (high nibble of the high byte)
        let nibble_2 = (full >> 8) & 0xF; // Second nibble (low nibble of the high byte)
        let nibble_3 = (full >> 4) & 0xF; // Third nibble (high nibble of the low byte)
        let nibble_4 = full & 0xF; // Fourth nibble (low nibble of the low byte)

        // println!("full   {:#016b} {:x}", full, full);
        // println!("nibble {:#016b} {:x}", nibble_1, nibble_1);
        // println!("nibble {:#016b} {:x}", nibble_2, nibble_2);
        // println!("nibble {:#016b} {:x}", nibble_3, nibble_3);
        // println!("nibble {:#016b} {:x}", nibble_4, nibble_4);
        self.context.increment_pc();

        match (nibble_1, nibble_2, nibble_3, nibble_4) {
            // Clear screen
            (0, 0, 0xE, 0) => {
                self.context.frame_buffer.clear();
            }
            // Return from subroutine
            (0, 0, 0xE, 0xE) => {
                let ret = self.context.stack_pop();
                self.context.pc = ret as usize;
            }
            (0, _, _, _) => {}
            // Set I to NNN
            (0xA, _, _, _) => {
                let masked = full & 0x0FFF;
                self.context.i = masked;
            }
            // Jump to NNN
            (1, _, _, _) => {
                let masked = full & 0x0FFF;
                self.context.pc = masked as usize;
            }
            // Jump to subroutine
            (2, _, _, _) => {
                let masked = full & 0x0FFF;
                self.context.stack_push(self.context.pc as u16);
                self.context.pc = masked as usize;
            }
            // Skip next if nn != vx
            (3, _, _, _) => {
                let nn = (full & 0x00FF) as u8;
                let vx = self.context.v[nibble_2 as usize];
                if nn == vx {
                    self.context.increment_pc();
                }
            }
            // Skip next if nn != vx
            (4, _, _, _) => {
                let nn = (full & 0x00FF) as u8;
                let vx = self.context.v[nibble_2 as usize];
                if nn != vx {
                    self.context.increment_pc();
                }
            }
            // Skip next if vx == vy
            (5, _, _, _) => {
                let vx = self.context.v[nibble_2 as usize];
                let vy = self.context.v[nibble_3 as usize];

                if vx == vy {
                    self.context.increment_pc();
                }
            }
            // Set vx to NN
            (6, _, _, _) => {
                self.context.v[nibble_2 as usize] = end;
            }
            // Add vx to NN
            (7, _, _, _) => {
                self.context.v[nibble_2 as usize] =
                    self.context.v[nibble_2 as usize].wrapping_add(end);
            }
            // Set vx to vy
            (8, _, _, 0) => {
                let vy = self.context.v[nibble_3 as usize];
                self.context.v[nibble_2 as usize] = vy;
            }
            // Skip next if vx != vy
            (9, _, _, _) => {
                let vx = self.context.v[nibble_2 as usize];
                let vy = self.context.v[nibble_3 as usize];

                if vx != vy {
                    self.context.increment_pc();
                }
            }
            // Wait for input and place in vx
            (0xF, _, 0, 0xA) => {
                let input = self.context.read_input();
                let x = nibble_2 as usize;

                if let Some(ch) = input {
                    println!("input {}, x {}", ch, x);
                    self.context.v[x] = ch;
                } else {
                    self.context.decrement_pc();
                }
            }
            // Set vx to delay
            (0xF, _, 0, 7) => {
                let x = nibble_2 as usize;
                self.context.v[x] = self.context.delay;
            }
            // Set delay timer
            (0xF, _, 1, 5) => {
                let x = nibble_2 as usize;
                self.context.delay = self.context.v[x];
            }
            // Set audio timer
            (0xF, _, 1, 8) => {
                let x = nibble_2 as usize;
                self.context.sound = self.context.v[x];
            }
            (0xF, _, 2, 9) => {
                let x = nibble_2 as usize;
                let val = self.context.v[x] * 5;

                self.context.i = (FONT_OFFSET + val) as u16;
            }
            (0xF, _, 1, 0xE) => {
                let val = self.context.v[nibble_2 as usize];
                let (res, overflowed) = self.context.i.overflowing_add(val as u16);
                self.context.i = res;
                if overflowed {
                    self.context.v[0x0F] = 1;
                }
            }

            // Draw to screen
            (0xD, _, _, _) => {
                let x = (self.context.v[nibble_2 as usize] % WIDTH as u8) as usize;
                let mut y = (self.context.v[nibble_3 as usize] % HEIGHT as u8) as usize;
                self.context.v[15] = 0;

                let i = self.context.i as usize;
                let end = (self.context.i + nibble_4) as usize;

                for byte in &self.context.memory[i..end] {
                    if y >= HEIGHT {
                        break;
                    }
                    let bits = (0..8).map(|i| (byte >> i) & 1).rev();
                    let mut x_row = x;
                    for bit in bits {
                        if x_row >= WIDTH {
                            break;
                        }
                        if bit != 0 {
                            let current_value = self
                                .context
                                .frame_buffer
                                .get_pixel(x_row, y)
                                .expect("Invalid position");

                            if current_value {
                                self.context.v[15] = 1;
                            }

                            self.context
                                .frame_buffer
                                .set_pixel(x_row, y, !current_value);
                        }
                        x_row += 1;
                    }
                    y += 1;
                }
            }
            _ => println!("Unknown operation: {:x}", full),
        }
    }
}
