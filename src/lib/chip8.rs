mod peripherals;
use peripherals::Peripherals;

use std::fs::File;
use std::io::prelude::*;

pub struct Opcode {
    pub op: u16
}

impl Opcode {

    pub fn opcode(& self) -> u8 {
        ((self.op & 0xF000) >> 12).try_into().unwrap()
    }

    pub fn x(&self) -> u8 {
        ((self.op & 0x0F00) >> 8).try_into().unwrap()
    }

    pub fn y(&self) -> u8 {
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
}

impl CPU {
    pub fn new() -> CPU {
        CPU {
            pc: 0x200,
            i: 0,
            v: vec![0; 16],
            memory: vec![0; 4096],
            stack: Vec::new(),
            delay_timer: 0,
            sound_timer: 0,
            peripherals: Peripherals::new(),
        }
    }

    pub fn get_byte(&self, n: u8, sprite: u8) -> bool {
        let mask: u8 = 1 << (7 - n);
        return (sprite & mask) > 0;
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
            0x0..=0xDF => {
                println!("Not implemented")
            }
            0xE1..=u8::MAX => {

            }
        }
        println!("OP code: {}", opcode.opcode());
    }
    fn op_0x1(&mut self, opcode: Opcode) {
        self.pc = opcode.nnn() as usize;
    }

    fn op_0x2(&mut self, opcode: Opcode) {
        println!("OP code: {}", opcode.opcode());
    }
    fn op_0x3(&mut self, opcode: Opcode) {
        println!("OP code: {}", opcode.opcode());
    }
    fn op_0x4(&mut self, opcode: Opcode) {
        println!("OP code: {}", opcode.opcode());
    }
    fn op_0x5(&mut self, opcode: Opcode) {
        println!("OP code: {}", opcode.opcode());
    }
    fn op_0x6(&mut self, opcode: Opcode) {
        self.v[opcode.x() as usize] = opcode.nn();
    }
    fn op_0x7(&mut self, opcode: Opcode) {
        self.v[opcode.x() as usize] += opcode.nn();
    }
    fn op_0x8(&mut self, opcode: Opcode) {
        println!("OP code: {}", opcode.opcode());
    }
    fn op_0x9(&mut self, opcode: Opcode) {
        println!("OP code: {}", opcode.opcode());
    }
    fn op_0xa(&mut self, opcode: Opcode) {
        self.i = opcode.nnn();
    }
    fn op_0xb(&mut self, opcode: Opcode) {
        println!("OP code: {}", opcode.opcode());
    }
    fn op_0xc(&mut self, opcode: Opcode) {
        println!("OP code: {}", opcode.opcode());
    }
    fn op_0xd(&mut self, opcode: Opcode) {
        let x: usize = (self.v[opcode.x() as usize] % 64) as usize;
        let y: usize = (self.v[opcode.y() as usize] % 32) as usize;
        self.v[0xF] = 0;

        for i in 0..opcode.n() {
            let sprite: u8 = self.memory[((self.i as usize) + i as usize)];
            for n in 0..8 {
                if self.get_byte(n, sprite) {
                    self.v[0xF] = self.peripherals.flip(x + n as usize, y + i as usize) as u8;
                }
            }
        }
    }

    fn op_0xe(&mut self, opcode: Opcode) {
        println!("OP code: {}", opcode.opcode());
    }
    fn op_0xf(&mut self, opcode: Opcode) {
        println!("OP code: {}", opcode.opcode());
    }


}

