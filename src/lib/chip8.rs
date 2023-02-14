mod peripherals;
use peripherals::Peripherals;
use rand::rngs::ThreadRng;

use std::fs::File;
use std::io::prelude::*;
use rand::Rng;

pub struct Opcode {
    pub op: u16
}

impl Opcode {

    pub fn opcode(& self) -> u8 {
        ((self.op & 0xF000) >> 12).try_into().unwrap()
    }

    pub fn x(&self) -> usize {
        ((self.op & 0x0F00) >> 8).try_into().unwrap()
    }

    pub fn y(&self) -> usize {
        ((self.op & 0xF0) >> 4).try_into().unwrap()
    }
    pub fn n(&self) -> u8 {
        ((self.op & 0xF)).try_into().unwrap()
    }
    pub fn nn(&self) -> u8 {
        ((self.op & 0xFF)).try_into().unwrap()
    }
    pub fn nnn(&self) -> u16 {
        ((self.op & 0xFFF)).try_into().unwrap()
    }
}

pub struct CPU {
    pub pc: usize,
    pub i: u16,
    pub v: Vec<u8>,
    pub memory: Vec<u8>,
    pub stack: Vec<u16>,
    pub delay_timer: u8,
    pub sound_timer: u8,
    pub peripherals: Peripherals,
    pub rnd: ThreadRng,
}

impl CPU {
    pub fn new() -> CPU {
        let mut cpu: CPU = CPU {
            pc: 0x200,
            i: 0,
            v: vec![0; 16],
            memory: vec![0; 4096],
            stack: Vec::new(),
            delay_timer: 0,
            sound_timer: 0,
            peripherals: Peripherals::new(),
            rnd: rand::thread_rng(),
        };
        cpu.add_fonts();
        cpu
    }

    pub fn get_byte(&self, n: u8, sprite: u8) -> bool {
        let mask: u8 = 1 << (7 - n);
        return (sprite & mask) > 0;
    }

    fn add_fonts(&mut self) {
        let font = vec! [
            0xF0, 0x90, 0x90, 0x90, 0xF0, // 0
            0x20, 0x60, 0x20, 0x20, 0x70, // 1
            0xF0, 0x10, 0xF0, 0x80, 0xF0, // 2
            0xF0, 0x10, 0xF0, 0x10, 0xF0, // 3
            0x90, 0x90, 0xF0, 0x10, 0x10, // 4
            0xF0, 0x80, 0xF0, 0x10, 0xF0, // 5
            0xF0, 0x80, 0xF0, 0x90, 0xF0, // 6
            0xF0, 0x10, 0x20, 0x40, 0x40, // 7
            0xF0, 0x90, 0xF0, 0x90, 0xF0, // 8
            0xF0, 0x90, 0xF0, 0x10, 0xF0, // 9
            0xF0, 0x90, 0xF0, 0x90, 0x90, // A
            0xE0, 0x90, 0xE0, 0x90, 0xE0, // B
            0xF0, 0x80, 0x80, 0x80, 0xF0, // C
            0xE0, 0x90, 0x90, 0x90, 0xE0, // D
            0xF0, 0x80, 0xF0, 0x80, 0xF0, // E
            0xF0, 0x80, 0xF0, 0x80, 0x80  // F
        ];

        for i in 0..font.len() {
            self.memory[0x50 + i] = font[i];
        }
    }

    /// Reads a program from filepath to memory.
    pub fn read_program(&mut self, filepath: String) -> std::result::Result<(), String> {
        let mut buffer = Vec::<u8>::new();
        match File::open(filepath) {
            Ok(mut file) => {
                file.read_to_end(&mut buffer).expect("Failed to read data.");
                if buffer.len() + 0x200 > self.memory.len() {
                    return Err("Program is larger than memory".to_string());
                }
                // TODO: There must be some better way to do this.
                for i in 0..buffer.len() {
                    self.memory[0x200 + i] = buffer[i];
                }
                return  Ok(());
            }
            Err(error) => {
                return Err(error.to_string());
            }
        }
    }

    fn overflow_add(&self, a: u8, b: u8) -> u8 {
        (((a as u16 + b as u16)) % (u8::MAX as u16 + 1)) as u8
    }
    fn overflow_subtract(&self, a: u8, b: u8) -> u8 {
        (((a as i16 - b as i16)) & (0xFF)) as u8
    }

    pub fn deincrement_timers(&mut self) {
        if self.delay_timer > 0 {
            self.delay_timer -= 1;
        }
        if self.sound_timer > 0 {
            self.sound_timer -= 1;
        }
    }

    /// Fetch the next Opcode and increment program counter.
    pub fn fetch(&mut self) -> Opcode {
        let op = Opcode{op: ((self.memory[self.pc] as u16) << 8) | (self.memory[self.pc+1] as u16)};
        self.pc += 2;
        return op;
    }
    /// Execute an opcode.
    pub fn execute(&mut self, opcode: Opcode) {
        match opcode.opcode() {
            0x0 => {
                self.op_0x0(opcode);
            }
            0x1 => {
                self.op_0x1(opcode);
            }
            0x2 => {
                self.op_0x2(opcode);
            }
            0x3 => {
                self.op_0x3(opcode);
            }
            0x4 => {
                self.op_0x4(opcode);
            }
            0x5 => {
                self.op_0x5(opcode);
            }
            0x6 => {
                self.op_0x6(opcode);
            }
            0x7 => {
                self.op_0x7(opcode);
            }
            0x8 => {
                self.op_0x8(opcode);
            }
            0x9 => {
                self.op_0x9(opcode);
            }
            0xA => {
                self.op_0xa(opcode);
            }
            0xB => {
                self.op_0xb(opcode);
            }
            0xC => {
                self.op_0xc(opcode);
            }
            0xD => {
                self.op_0xd(opcode);
            }
            0xE => {
                self.op_0xe(opcode);
            }
            0xF => {
                self.op_0xf(opcode);
            }
            16_u8..=u8::MAX => {
                println!("You shouldn't reach this...");
            }
        }
    }

    fn op_0x0(&mut self, opcode: Opcode) {
        match opcode.nn() {
            0xE0 => {
                self.peripherals.clear();
            }
            0xEE => {
                self.pc = self.stack.pop().unwrap() as usize;
            }
            0x0..=0xDF | 0xE1..=0xED | 0xEF..=u8::MAX => {
                println!("Not implemented")
            }
        }
    }

    fn op_0x1(&mut self, opcode: Opcode) {
        self.pc = opcode.nnn() as usize;
    }

    fn op_0x2(&mut self, opcode: Opcode) {
        self.stack.push(self.pc as u16);
        self.pc = opcode.nnn() as usize;
    }

    fn op_0x3(&mut self, opcode: Opcode) {
        if self.v[opcode.x()] == opcode.nn() {
            self.pc += 2;
        }
    }

    fn op_0x4(&mut self, opcode: Opcode) {
        if self.v[opcode.x()] != opcode.nn() {
            self.pc += 2;
        }
    }

    fn op_0x5(&mut self, opcode: Opcode) {
        if self.v[opcode.x()] == self.v[opcode.y()] {
            self.pc += 2;
        }
    }

    fn op_0x6(&mut self, opcode: Opcode) {
        self.v[opcode.x()] = opcode.nn();
    }

    fn op_0x7(&mut self, opcode: Opcode) {
        self.v[opcode.x()] = self.overflow_add(self.v[opcode.x()], opcode.nn());
    }

    fn op_0x8(&mut self, opcode: Opcode) {
        match opcode.n() {
            0x0 => {
                self.v[opcode.x()] = self.v[opcode.y()];
            }
            0x1 => {
                self.v[opcode.x()] |= self.v[opcode.y()];
            }
            0x2 => {
                self.v[opcode.x()] &= self.v[opcode.y()];
            }
            0x3 => {
                self.v[opcode.x()] ^= self.v[opcode.y()];
            }
            0x4 => {
                if self.v[opcode.x()] as u16 + self.v[opcode.y()] as u16 > u8::MAX.into() {
                    self.v[0xF] = 1;
                }
                self.v[opcode.x()] = self.overflow_add(self.v[opcode.x()], self.v[opcode.y()]);
            }
            0x5 => {
                if self.v[opcode.x()] > self.v[opcode.y()] {
                    self.v[0xF] = 1;
                }
                self.v[opcode.x()] = self.overflow_subtract( self.v[opcode.x()], self.v[opcode.y()]);
            }
            0x6 => {
                if self.v[opcode.y()] & 0x1 > 0 {
                    self.v[0xF] = 1;
                }
                self.v[opcode.y()] = self.v[opcode.y()] >> 1;
            }
            0x7 => {
                if self.v[opcode.y()] > self.v[opcode.x()] {
                    self.v[0xF] = 1;
                }
                self.v[opcode.x()] = self.overflow_subtract(self.v[opcode.y()], self.v[opcode.x()]);
            }
            0xE => {
                if self.v[opcode.y()] & 0b10000000 > 0 {
                    self.v[0xF] = 1;
                }
                self.v[opcode.x()] = self.v[opcode.y()] << 1;
            }
            8_u8..=13_u8 | 15_u8..=u8::MAX => {
                print!("Unknown command")
            }
        }
    }

    fn op_0x9(&mut self, opcode: Opcode) {
        if self.v[opcode.x()] != self.v[opcode.y()] {
            self.pc += 2;
        }
    }

    fn op_0xa(&mut self, opcode: Opcode) {
        self.i = opcode.nnn();
    }

    fn op_0xb(&mut self, opcode: Opcode) {
        self.pc = (opcode.nnn() + (self.v[0] as u16)) as usize;
    }

    fn op_0xc(&mut self, opcode: Opcode) {
        let rndnum: u8 = self.rnd.gen();
        self.v[opcode.x()] = opcode.nn() & rndnum;
    }

    fn op_0xd(&mut self, opcode: Opcode) {
        let x: usize = (self.v[opcode.x()] % 64) as usize;
        let y: usize = (self.v[opcode.y()] % 32) as usize;
        self.v[0xF] = 0;

        for i in 0..opcode.n() as usize {
            let sprite: u8 = self.memory[((self.i as usize) + i)];
            for n in 0..8 {
                if self.get_byte(n, sprite) {
                    self.v[0xF] = self.peripherals.flip(x + n as usize, y + i) as u8;
                }
            }
        }
    }

    fn op_0xe(&mut self, opcode: Opcode) {
        match opcode.nn() {
            0x9E => {
                if self.peripherals.key_states[self.v[opcode.x() as usize] as usize] {
                    self.pc += 2;
                }
            }
            0xA1 => {
                if !self.peripherals.key_states[self.v[opcode.x() as usize] as usize] {
                    self.pc += 2;
                }
            }
            _ => {
                println!("Not implemented")
            }

        }
    }

    fn op_0xf(&mut self, opcode: Opcode) {
        match opcode.nn() {
            0x07 => {
                self.v[opcode.x()] = self.delay_timer;
            }
            0x15 => {
                self.delay_timer = self.v[opcode.x()];
            }
            0x18 => {
                self.sound_timer = self.v[opcode.x()];
            }
            0x1E => {
                self.i += self.v[opcode.x()] as u16;
            }
            0x0A => {
                for n in 0..0xF as usize {
                    if self.peripherals.key_states[n] {
                        self.v[opcode.x() as usize] = n as u8;
                        return;
                    }
                }
                self.pc -= 2;
            }
            0x29 => {
                self.i = (0x50 + opcode.x() * 5).try_into().unwrap();
            }

            0x33 => {
                self.memory[(self.i + 2) as usize] = (self.v[opcode.x()] % 10) as u8;
                self.memory[(self.i + 1) as usize] = ((self.v[opcode.x()] / 10) % 10) as u8;
                self.memory[(self.i + 0) as usize] = ((self.v[opcode.x()] / 100) % 10) as u8;
            }
            0x55 => {
                for i in 0..=opcode.x() {
                    self.memory[self.i as usize + i] = self.v[i];
                }
            }
            0x65 => {
                for i in 0..=opcode.x() {
                    self.v[i] = self.memory[self.i as usize + i];
                }
            }
            _ => {
                println!("Not implemented")
            }
        }
    }


}
