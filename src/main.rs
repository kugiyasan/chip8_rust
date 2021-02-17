use chip8::Chip8;
use std::env;

mod chip8;
mod display;
mod memory;

fn main() {
    let mut args = env::args();
    args.next();
    let path = match args.next() {
        Some(p) => p,
        None => "../../Projects/chip8/roms/IBM Logo.ch8".to_string(),
    };

    let mut chip8 = Chip8::new();
    match chip8.load(&path) {
        Ok(_) => chip8.run(),
        Err(err) => eprintln!("Can't load the ch8 file: {}", err),
    };
}
