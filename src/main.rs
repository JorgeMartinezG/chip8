use std::fs::File;
use std::io::Read;

use chip8::Chip8;

mod ram;
mod chip8;
mod cpu;


fn main() {
    // Read file and load into VM memory.
    let mut file = File::open("roms/PONG").unwrap();
    let mut data = Vec::<u8>::new();

    file.read_to_end(&mut data);

    let mut chip8 = Chip8::new();
    chip8.load_rom(&data);

    chip8.run_instruction();
    chip8.run_instruction();
    chip8.run_instruction();
    println!("{:?}", data);

}
