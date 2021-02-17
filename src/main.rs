mod chip8;
mod display;
mod memory;

fn main() {
    let mut chip8 = chip8::Chip8::new();
    match chip8.load("../../Projects/chip8/roms/IBM Logo.ch8") {
        Ok(_) => chip8.run(),
        Err(err) => panic!(err),
    };
}
