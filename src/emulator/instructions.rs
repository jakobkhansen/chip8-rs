use rand::Rng;

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

        // println!("{}", self.context.pc);
        // println!(
        //     "{:#06x} {}: full   {:#016b} {:x}",
        //     self.context.pc, self.context.pc, full, full
        // );
        // let hex_string: String = self
        //     .context
        //     .v
        //     .iter()
        //     .map(|b| format!("{:02X}", b))
        //     .collect::<Vec<_>>() // Collect into Vec<String>
        //     .join(" ");
        // println!("i={} {}\n", self.context.i, hex_string);
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
            (8, _, _, 1) => {
                let vx = self.context.v[nibble_2 as usize];
                let vy = self.context.v[nibble_3 as usize];
                self.context.v[nibble_2 as usize] = vx | vy;
            }
            (8, _, _, 2) => {
                let vx = self.context.v[nibble_2 as usize];
                let vy = self.context.v[nibble_3 as usize];
                self.context.v[nibble_2 as usize] = vx & vy;
            }
            (8, _, _, 3) => {
                let vx = self.context.v[nibble_2 as usize];
                let vy = self.context.v[nibble_3 as usize];
                self.context.v[nibble_2 as usize] = vx ^ vy;
            }
            (8, _, _, 4) => {
                let vx = self.context.v[nibble_2 as usize];
                let vy = self.context.v[nibble_3 as usize];
                let (res, overflow) = vx.overflowing_add(vy);
                self.context.v[nibble_2 as usize] = res;
                if overflow {
                    self.context.v[0x0F] = 1;
                }
            }
            (8, _, _, 5) => {
                let vx = self.context.v[nibble_2 as usize];
                let vy = self.context.v[nibble_3 as usize];
                let (res, overflow) = vx.overflowing_sub(vy);
                self.context.v[nibble_2 as usize] = res;

                self.context.v[0x0F] = 1;

                if overflow {
                    self.context.v[0x0F] = 0;
                }
            }
            (8, _, _, 6) => {
                let x = nibble_2 as usize;
                let y = nibble_3 as usize;
                self.context.v[x] = self.context.v[y];

                self.context.v[0x0F] = self.context.v[x] & 0b10000000;
                self.context.v[x] <<= 1;
            }
            (8, _, _, 7) => {
                let vx = self.context.v[nibble_2 as usize];
                let vy = self.context.v[nibble_3 as usize];
                let (res, overflow) = vy.overflowing_sub(vx);
                self.context.v[nibble_2 as usize] = res;

                self.context.v[0x0F] = 1;

                if overflow {
                    self.context.v[0x0F] = 0;
                }
            }
            (8, _, _, 0xE) => {
                let x = nibble_2 as usize;
                let y = nibble_3 as usize;
                self.context.v[x] = self.context.v[y];
                self.context.v[0x0F] = self.context.v[x] & 0b00000001;
                self.context.v[x] >>= 1;
            }
            // Skip next if vx != vy
            (9, _, _, _) => {
                let vx = self.context.v[nibble_2 as usize];
                let vy = self.context.v[nibble_3 as usize];

                if vx != vy {
                    self.context.increment_pc();
                }
            }
            // Set I to NNN
            (0xA, _, _, _) => {
                let masked = full & 0x0FFF;
                self.context.i = masked;
            }
            // Jump with offset
            (0xB, _, _, _) => {
                let nnn = full & 0x0FFF;
                let v0 = self.context.v[0] as u16;
                self.context.pc = (nnn + v0) as usize;
            }
            // Random
            (0xC, _, _, _) => {
                let nn = (full & 0x00FF) as u8;
                let mut rng = rand::thread_rng();
                let generated: u8 = rng.r#gen();
                self.context.v[nibble_2 as usize] = generated & nn;
            }
            (0xE, _, 9, 0xE) => {
                let x = self.context.v[nibble_2 as usize];
                if self.context.held_keys[x as usize] {
                    self.context.pc += 2;
                }
            }
            (0xE, _, 0xA, 1) => {
                let x = self.context.v[nibble_2 as usize];
                if !self.context.held_keys[x as usize] {
                    self.context.pc += 2;
                }
            }
            // Wait for input and place in vx
            (0xF, _, 0, 0xA) => {
                let input = self.context.read_input();
                let x = nibble_2 as usize;

                if let Some(ch) = input {
                    self.context.v[x] = ch;
                } else {
                    self.context.decrement_pc();
                }
            }
            // Set delay timer
            (0xF, _, 1, 5) => {
                let x = nibble_2 as usize;
                self.context.delay = self.context.v[x];
            }
            // Set vx to delay
            (0xF, _, 0, 7) => {
                let x = nibble_2 as usize;
                self.context.v[x] = self.context.delay;
            }
            // Set audio timer
            (0xF, _, 1, 8) => {
                let x = nibble_2 as usize;
                self.context.sound = self.context.v[x];
            }
            // Add X to I
            (0xF, _, 1, 0xE) => {
                let val = self.context.v[nibble_2 as usize];
                let (res, overflowed) = self.context.i.overflowing_add(val as u16);
                self.context.i = res;
                if overflowed {
                    self.context.v[0x0F] = 1;
                }
            }
            // Set I to font character address
            (0xF, _, 2, 9) => {
                let x = nibble_2 as usize;
                let val = (self.context.v[x] as u16) * 5;

                self.context.i = (FONT_OFFSET as u16) + val;
            }
            // Store v[0] to v[x] in memory (from I)
            (0xF, _, 5, 5) => {
                let x = nibble_2;

                for i in 0..(x + 1) {
                    self.context.memory[(self.context.i + i) as usize] = self.context.v[i as usize];
                }
            }
            // Store memory from I in v[0] to v[x]
            (0xF, _, 6, 5) => {
                let x = nibble_2;

                for i in 0..(x + 1) {
                    self.context.v[i as usize] = self.context.memory[(self.context.i + i) as usize];
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
