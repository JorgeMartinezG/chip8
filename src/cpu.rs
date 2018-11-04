use ram::Ram;

pub const PROGRAM_START_ADDRESS: u16 = 0x200;

pub struct Cpu {
    vx: [u8; 16],
    pc: u16,
    i: u16,
    ret_stack: Vec<u16>,
}

impl Cpu {
	pub fn new() -> Cpu {
        Cpu {
            vx: [0; 16],
            pc: PROGRAM_START_ADDRESS,
            i: 0,
            ret_stack: Vec::<u16>::new(),
		}
	}

	pub fn fetch_instruction(&mut self, ram: &Ram) -> u16 {
		let hi = ram.read_byte(self.pc) as u16;
        let lo = ram.read_byte(self.pc + 1) as u16;
		let instruction: u16 = (hi << 8) | lo;

        println!(
            "Instruction read at {:#X} = {:#X} - hi:{:?} lo:{:?} ",
            self.pc,
            instruction,
            hi,
            lo
		);

		self.pc += 2;
		instruction
	}
}