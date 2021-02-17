use std::fs::File;
use std::io::{Read, Result};

pub struct Memory {
    // The ram allocated for the program
    pub ram: [u8; 4096],
    // v contains 16 general purpose 8-bit registers
    pub v: [u8; 16],
    // i stores the memory addresses
    pub i: u16,
    // dt is the delay timer
    pub dt: u8,
    // st is the sound timer
    pub st: u8,
    // pc is the program counter
    pub pc: u16,
    // sp is the stack pointer
    pub sp: u8,
    // The stack stores the addresses of the running subroutines
    pub stack: [u16; 16],
}

impl Memory {
    pub fn new() -> Memory {
        Memory {
            ram: [0; 4096],
            v: [0; 16],
            i: 0,
            dt: 0,
            st: 0,
            pc: 0x200,
            sp: 0,
            stack: [0; 16],
        }
    }

    fn load_rom(&mut self, path: &str) -> Result<()> {
        File::open(path)?.read(&mut self.ram[(self.pc as usize)..])?;
        Ok(())
    }

    fn load_digits(&mut self) {
        let digits = [
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

        digits
            .iter()
            .flatten()
            .enumerate()
            .for_each(|(i, &b)| self.ram[i] = b);
    }

    pub fn load(&mut self, path: &str) -> Result<()> {
        self.load_rom(path)?;
        self.load_digits();
        Ok(())
    }
}
