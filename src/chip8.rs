use ram::Ram;
use cpu;
use cpu::Cpu;

pub struct Chip8 {
	ram: Ram,
	cpu: Cpu
}

struct Decoded {
	nnn: u16,
	nn: u8,
	n: u8,
	x: u8,
	y: u8
}

fn decode_instruction(instruction: u16) -> Decoded {
	Decoded {
	    nnn: instruction & 0x0FFF;
        nn: (instruction & 0x0FF) as u8;
        n: (instruction & 0x00F) as u8;
        x: ((instruction & 0x0F00) >> 8) as u8;
		y: ((instruction & 0x00F0) >> 4) as u8;
	}
}


impl Chip8 {
	pub fn new() -> Chip8 {
		let mut chp8 = Chip8 {
			ram: Ram::new(),
			cpu: Cpu::new()
		};
		chp8.load_sprites();

		chp8
	}

	pub fn load_rom(&mut self, data: &[u8]) {
		for index in 0..data.len() {
			self.ram.write_byte(cpu::PROGRAM_START_ADDRESS + (index as u16), data[index]);
		}
	}

	pub fn run_instruction(&mut self) {
		let instruction = self.cpu.fetch_instruction(&self.ram);
		let decoded = decode_instruction(instruction);
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