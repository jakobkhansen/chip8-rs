use super::emulator::Chip8Emulator;

impl Chip8Emulator {
    pub fn execute_instruction(&mut self) {
        let (start, end) = self.context.get_next_instruction();
        let full = ((start as u16) << 8) | (end as u16);
        // let nibbles = [
        //     full & 0xF000,
        //     full & 0xFF00,
        //     full & 0xFFF0,
        //     full & 0x000F,
        //     full & 0x00FF,
        //     full & 0x0FFF,
        // ];
        let nibble_1 = (full >> 12) & 0xF; // First nibble (high nibble of the high byte)
        let nibble_2 = (full >> 8) & 0xF; // Second nibble (low nibble of the high byte)
        let nibble_3 = (full >> 4) & 0xF; // Third nibble (high nibble of the low byte)
        let nibble_4 = full & 0xF; // Fourth nibble (low nibble of the low byte)

        println!("full   {:#016b} {:x}", full, full);
        println!("nibble {:#016b} {:x}", nibble_1, nibble_1);
        println!("nibble {:#016b} {:x}", nibble_2, nibble_2);
        println!("nibble {:#016b} {:x}", nibble_3, nibble_3);
        println!("nibble {:#016b} {:x}", nibble_4, nibble_4);
        self.context.increment_pc();

        match (nibble_1, nibble_2, nibble_3, nibble_4) {
            (0, 0, 0xE, 0) => println!("Clear screen"),
            (0xA, _, _, _) => println!("Set index register I"),
            (1, _, _, _) => println!("Jump"),
            (6, _, _, _) => println!("Set V{} to {}", nibble_2, end),
            (7, _, _, _) => println!("Add {} to V{}", end, nibble_2),
            (0xD, _, _, _) => println!("Draw to screen"),
            _ => println!("Unknown operation"),
        }
        println!();
    }
}
