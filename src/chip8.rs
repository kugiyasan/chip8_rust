use display::Display;
use memory::Memory;
use rand::Rng;
use std::io::Result;
use std::{thread, time};

use crate::display;
use crate::memory;

pub struct Chip8 {
    memory: memory::Memory,
    display: display::Display,
}

impl Chip8 {
    pub fn new() -> Self {
        Self {
            memory: Memory::new(),
            display: Display::new(),
        }
    }

    pub fn load(&mut self, path: &str) -> Result<()> {
        self.memory.load(path)
    }

    fn get_next_instruction(&self) -> u16 {
        let b1 = self.memory.ram[self.memory.pc as usize];
        let b2 = self.memory.ram[self.memory.pc as usize + 1];
        (b1 as u16) << 8 | b2 as u16
    }

    pub fn run(&mut self) {
        loop {
            let instruction = self.get_next_instruction();

            if instruction & 0xF000 == 0x1000 && instruction & 0x0FFF == self.memory.pc {
                println!("Encountered an infinite loop, breaking from the loop");
                break;
            }
            
            self.execute_intruction(instruction);
            self.memory.pc += 2;

            // 1 / 500Hz = 2ms
            thread::sleep(time::Duration::from_millis(2));
        }
    }

    fn draw_sprites(&mut self, vx: u8, vy: u8, nibble: u8) {
        self.memory.v[0xF] = 0;

        let vx = vx as usize;
        let vy = vy as usize;
        let nibble = nibble as usize;
        let i = self.memory.i as usize;

        for n in 0..nibble {
            let byte = self.memory.ram[i + 8 * n];

            for x in 0..8 {
                let pixel = (byte >> (7 - x)) == 1;
                if self.display.screen[vy + n][vx + x] && pixel {
                    self.memory.v[0xF] = 1;
                }
                self.display.screen[vy + n][vx + x] ^= pixel;
            }
        }
        self.display.print_screen();
    }

    pub fn execute_intruction(&mut self, instruction: u16) {
        let mut mem = &mut self.memory;

        let nnn = instruction & 0x0FFF;
        let n = (instruction & 0x000F) as u8;
        let x = (instruction & 0x0F00 >> 8) as usize;
        let y = (instruction & 0x00F0 >> 4) as usize;
        let kk = (instruction & 0x00FF) as u8;

        match instruction & 0xF000 {
            0x0000 => match nnn {
                0x00E0 => self.display.clear(),
                0x00EE => {
                    mem.pc = mem.stack[mem.sp as usize];
                    mem.sp -= 1;
                }
                _ => panic!("Unknown instruction: {}", instruction),
            },
            0x1000 => mem.pc = nnn - 2,
            0x2000 => {
                mem.sp += 1;
                mem.stack[mem.sp as usize] = mem.pc;
                mem.pc = nnn;
            }
            0x3000 => {
                if mem.v[x] == kk {
                    mem.pc += 2
                }
            }
            0x4000 => {
                if mem.v[x] != kk {
                    mem.pc += 2
                }
            }
            0x5000 => {
                if mem.v[x] == mem.v[y] {
                    mem.pc += 2
                }
            }
            0x6000 => mem.v[x] = kk,
            0x7000 => mem.v[x] += kk,
            0x8000 => match n {
                0x0000 => mem.v[x] = mem.v[y],
                0x0001 => mem.v[x] |= mem.v[y],
                0x0002 => mem.v[x] &= mem.v[y],
                0x0003 => mem.v[x] ^= mem.v[y],
                0x0004 => {
                    mem.v[0xF] = (mem.v[x] > u8::MAX - mem.v[y]) as u8;
                    mem.v[x] += mem.v[y];
                }
                0x0005 | 0x0007 => {
                    mem.v[0xF] = (mem.v[x] > u8::MAX + mem.v[y]) as u8;
                    mem.v[x] -= mem.v[y];
                }
                0x0006 => {
                    mem.v[0xF] = mem.v[x] & 0x01;
                    mem.v[x] >>= 1;
                }
                0x000E => {
                    mem.v[x] <<= 1;
                }
                _ => panic!("Unknown instruction: {}", instruction),
            },
            0x9000 => {
                if mem.v[x] != mem.v[y] {
                    mem.pc += 2
                }
            }
            0xA000 => mem.i = nnn,
            0xB000 => mem.pc = nnn + mem.v[0] as u16,
            0xC000 => mem.v[x] = rand::thread_rng().gen::<u8>() & kk,
            0xD000 => {
                let vx = mem.v[x];
                let vy = mem.v[y];
                self.draw_sprites(vx, vy, n);
            }
            0xE000 => match kk {
                0x009E => (), // !
                0x00A1 => (), // !
                _ => panic!("Unknown instruction: {}", instruction),
            },
            0xF000 => match kk {
                0x0007 => mem.v[x] = mem.dt,
                0x000A => mem.v[x] = 0, // ! should wait for a keypress
                0x0015 => mem.dt = mem.v[x],
                0x0018 => mem.st = mem.v[x],
                0x001E => mem.i += mem.v[x] as u16,
                0x0029 => mem.i = mem.v[x] as u16 * 5,
                0x0033 => (), // !
                0x0055 => {
                    for i in 0..x {
                        mem.ram[mem.i as usize + i] = mem.v[i];
                    }
                }
                0x0065 => {
                    for i in 0..x {
                        mem.v[i] = mem.ram[mem.i as usize + i];
                    }
                }
                _ => panic!("Unknown instruction: {}", instruction),
            },
            _ => panic!("Unknown instruction: {}", instruction),
        }
    }
}
