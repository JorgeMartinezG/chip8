use ram::Ram;
use cpu;
use cpu::Cpu;
use display::Display;
use keyboard::Keyboard;

pub struct Chip8 {
	ram: Ram,
	cpu: Cpu,
    display: Display,
    keyboard: Keyboard
}


impl Chip8 {
	pub fn new() -> Chip8 {
		let mut chp8 = Chip8 {
			ram: Ram::new(),
			cpu: Cpu::new(),
            display: Display::new(),
            keyboard: Keyboard::new()
		};
		chp8.load_sprites();

		chp8
	}

    pub fn get_display_buffer(&self) -> &[u8] {
        &self.display.get_buffer()
    }


	pub fn load_rom(&mut self, data: &[u8]) {
		for index in 0..data.len() {
			self.ram.write_byte(cpu::PROGRAM_START_ADDRESS + (index as u16), data[index]);
		}
	}

	pub fn run_instruction(&mut self) {
		self.cpu.decrement_timer();

		let instruction = self.cpu.fetch_instruction(&self.ram);
		let decoded = self.cpu.decode_instruction(instruction);

		self.cpu.execute_instruction(&mut self.ram, instruction, decoded, &self.keyboard, &mut self.display);
		println!("Cpu state: {:?}", self.cpu);
	}

    pub fn set_key_pressed(&mut self, key: Option<u8>) {
		println!{"Key pressed!!!!! --> {:?}", Some(key)};
        self.keyboard.set_key_pressed(key);
	}

	fn load_sprites(&mut self) {
        //Initialize memory with the predefined sprites from 0, 1, 2 ... F
        let sprites: [[u8; 5]; 16] = [
            [0xF0, 0x90, 0x90, 0x90, 0xF0],
            [0x20, 0x60, 0x20, 0x20, 0x70],
            [0xF0, 0x10, 0xF0, 0x80, 0xF0],
            [0xF0, 0x10, 0xF0, 0x10, 0xF0],
            [0x90, 0x90, 0xF0, 0x10, 0x10],
            [0xF0, 0x80, 0xF0, 0x10, 0xF0],
            [0xF0, 0x80, 0xF0, 0x90, 0xF0],
            [0xF0, 0x10, 0x20, 0x40, 0x40],
            [0xF0, 0x90, 0xF0, 0x90, 0xF0],
            [0xF0, 0x90, 0xF0, 0x10, 0xF0],
            [0xF0, 0x90, 0xF0, 0x90, 0x90],
            [0xE0, 0x90, 0xE0, 0x90, 0xE0],
            [0xF0, 0x80, 0x80, 0x80, 0xF0],
            [0xE0, 0x90, 0x90, 0x90, 0xE0],
            [0xF0, 0x80, 0xF0, 0x80, 0xF0],
            [0xF0, 0x80, 0xF0, 0x80, 0x80],
        ];

        let mut i = 0;
        for sprite in &sprites {
            for ch in sprite {
                self.ram.write_byte(i as u16, *ch);
                i += 1;
            }
		}
	}

}