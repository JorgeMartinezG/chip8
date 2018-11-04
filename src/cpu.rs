use ram::Ram;
use std::fmt;

pub const PROGRAM_START_ADDRESS: u16 = 0x200;

pub struct Cpu {
    vx: [u8; 16],
    pc: u16,
    i: u16,
    stack: [u16; 16],
    sp: u8,
}

pub struct Decode {
    nnn: u16,
    nn: u8,
    n: u8,
    x: u8,
    y: u8
}

impl Cpu {
	pub fn new() -> Cpu {
        Cpu {
            vx: [0; 16],
            pc: PROGRAM_START_ADDRESS,
            i: 0,
            stack: [0; 16],
            sp:0
		}
	}

	pub fn fetch_instruction(&self, ram: &Ram) -> u16 {
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

		instruction
	}

    pub fn decode_instruction(&self, instruction: u16) -> Decode {
        Decode {
            nnn: instruction & 0x0FFF,
            nn: (instruction & 0x0FF) as u8,
            n: (instruction & 0x00F) as u8,
            x: ((instruction & 0x0F00) >> 8) as u8,
            y: ((instruction & 0x00F0) >> 4) as u8,
        }        
    }

    pub fn draw_sprite(&self, ram:&Ram, x:u8, y:u8, n:u8) {
        for j in 0..n {
            let mut b = ram.read_byte(self.i + (j as u16));
            for _ in 0..8 {
                match (b & 0b1000_0000) >> 7 {
                    0 => print!("_"),
                    1 => print!("#"),
                    _ => unreachable!()
                }
                b = b << 1;
            }
            print!("\n");
        }

    }


    pub fn execute_instruction(&mut self, ram: &mut Ram, instruction: u16, decode: Decode) {
        match (instruction & 0xF000) >> 12 {
            0x0 => {
                match decode.nn {
                    // 0xE0 => {
                    //     bus.clear_screen();
                    //     self.pc += 2;
                    // }
                    0xEE => {
                        //return from subroutine
                        if self.sp > 0 {
                            self.sp -= 1;
                        }

                        self.pc = self.stack[self.sp as usize];
                    }
                    _ => panic!(
                        "Unrecognized 0x00** instruction {:#X}:{:#X}",
                        self.pc,
                        instruction
                    ),
                }
            },
            0x1 => {
                //goto nnn;
                self.pc = decode.nnn;
            }
            0x2 => {
                self.stack[(self.sp + 2) as usize] = self.pc;
                self.pc = decode.nnn;
            }
            0x3 => {
                //if(Vx==NN)
                let vx = self.read_vx(decode.x);
                self.pc += if vx == decode.nn {4} else {2}
            }
            0x4 => {
                //Skip next instruction if(Vx!=NN)
                let vx = self.read_vx(decode.x);
                self.pc += if vx != decode.nn {4} else {2}
            }
            0x5 => {
                //Skip next instruction if(Vx==Vy)
                let vx = self.read_vx(decode.x);
                let vy = self.read_vx(decode.y);
                if vx == vy {
                    self.pc += 4;
                } else {
                    self.pc += 2;
                }
            }
            0x6 => {
                //vx = nn
                self.write_vx(decode.x, decode.nn);
                self.pc += 2;
            },
            0x7 => {
                let vx = self.read_vx(decode.x);
                self.write_vx(decode.x, vx.wrapping_add(decode.nn));
                self.pc += 2;
            }
            0x8 => {
                let vy = self.read_vx(decode.y);
                let vx = self.read_vx(decode.x);

                match decode.n {
                    0x0 => {
                        // Vx=Vy
                        self.write_vx(decode.x, vy);
                    }
                    0x2 => {
                        // Vx=Vx&Vy
                        self.write_vx(decode.x, vx & vy);
                    }
                    0x3 => {
                        // Vx=Vx^Vy
                        self.write_vx(decode.x, vx ^ vy);
                    }
                    0x4 => {
                        //  Vx += Vy
                        let sum: u16 = vx as u16 + vy as u16;
                        self.write_vx(decode.x, sum as u8);
                        if sum > 0xFF {
                            self.write_vx(0xF, 1);
                        }
                    }
                    0x5 => {
                        let diff: i8 = vx as i8 - vy as i8;
                        self.write_vx(decode.x, diff as u8);
                        if diff < 0 {
                            self.write_vx(0xF, 1);
                        } else {
                            self.write_vx(0xF, 0);
                        }
                    }
                    0x6 => {
                        // Vx=Vx>>1
                        self.write_vx(0xF, vx & 0x1);
                        self.write_vx(decode.x, vx >> 1);
                    }
                    0x7 => {
                        let diff: i8 = vy as i8 - vx as i8;
                        self.write_vx(decode.x, diff as u8);
                        if diff < 0 {
                            self.write_vx(0xF, 1);
                        } else {
                            self.write_vx(0xF, 0);
                        }
                    }
                    0xE => {
                        // VF is the most significant bit value.
                        // SHR Vx
                        self.write_vx(0xF, (vx & 0x80) >> 7);
                        self.write_vx(decode.x, vx << 1);
                    }
                    _ => panic!(
                        "Unrecognized 0x8XY* instruction {:#X}:{:#X}",
                        self.pc,
                        instruction
                    ),
                };

                self.pc += 2;

            }
            0xA => {
                // i = NNN
                self.i = decode.nnn;
                self.pc += 2;
            },
            0xD => {
                // Draw sprite
                //draw(Vx,Vy,N)
                self.draw_sprite(ram, decode.x, decode.y, decode.n);
                self.pc += 2;
            }
            // 0xE => {
            //     match nn {
            //         0xA1 => {
            //             // if(key()!=Vx) then skip the next instruction
            //             let key = self.read_reg_vx(x);
            //             if !bus.is_key_pressed(key) {
            //                 self.pc += 4;
            //             } else {
            //                 self.pc += 2;
            //             }
            //         }
            //         0x9E => {
            //             // if(key()==Vx) then skip the next instruction
            //             let key = self.read_reg_vx(x);
            //             if bus.is_key_pressed(key) {
            //                 self.pc += 4;
            //             } else {
            //                 self.pc += 2;
            //             }
            //         }
            //         _ => panic!(
            //             "Unrecognized 0xEX** instruction {:#X}:{:#X}",
            //             self.pc,
            //             instruction
            //         ),
            //     };
            // }
            0xF => {
                match decode.nn {
                    // 0x07 => {
                    //     self.write_reg_vx(x, bus.get_delay_timer());
                    //     self.pc += 2;
                    // }
                    // 0x0A => {
                    //     if let Some(val) = bus.get_key_pressed() {
                    //         self.write_reg_vx(x, val);
                    //         self.pc += 2;
                    //     }
                    // }
                    // 0x15 => {
                    //     bus.set_delay_timer(self.read_reg_vx(x));
                    //     self.pc += 2;
                    // }
                    0x18 => {
                        // TODO Sound timer
                        self.pc += 2;
                    }
                    0x1E => {
                        //I +=Vx
                        let vx = self.read_vx(decode.x);
                        self.i += vx as u16;
                        self.pc += 2;
                    }
                    0x29 => {
                        self.i = self.read_vx(decode.x) as u16 * 5;
                        self.pc += 2;
                    }

                    0x33 => {
                        let vx = self.read_vx(decode.x);
                        ram.write_byte(self.i, vx / 100);
                        ram.write_byte(self.i + 1, (vx % 100) / 10);
                        ram.write_byte(self.i + 2, vx % 10);

                        self.pc += 2;
                    },
                    0x65 => {
                        for index in 0..decode.x + 1 {
                            let value = ram.read_byte(self.i + index as u16);
                            self.write_vx(index, value);
                        }
                        //self.i += x as u16 + 1;
                        self.pc += 2;
                    }
                    _ => panic!("Unrecognized instruction {:#X}:{:#X}", self.pc, instruction)                    
                }


            }
            _ => panic!("Unrecognized instruction {:#X}:{:#X}", self.pc, instruction)

        }
    }

    pub fn read_vx(&self, idx:u8) -> u8{
        self.vx[idx as usize]
    }

    pub fn write_vx(&mut self, idx:u8, nn:u8) {
        self.vx[idx as usize] = nn;
    }

}

impl fmt::Debug for Cpu {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        
        write!(f, "pc: {:#X}\n", self.pc);
        write!(f, "vx: ");
        for item in self.vx.iter() {
            write!(f, "{:#X} ", *item);
        }
        write!(f, "\n");
        write!(f, "i: {:#X}\n", self.i)
    }
}