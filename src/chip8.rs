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
        ((self.op & 0xFFF) >> 2).try_into().unwrap()
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
}

impl CPU {
    pub fn new() -> CPU {
        CPU {
            pc: 0x200,
            i: 0,
            v: Vec::new(),
            memory: Vec::new(),
            stack: Vec::new(),
            delay_timer: 0,
            sound_timer: 0
        }
    }

    pub fn read_program(&mut self, filepath: String) {
        
    }

    pub fn fetch(&mut self) -> Opcode {
        let op = Opcode{op: ((self.memory[self.pc] as u16) << 8) | (self.memory[self.pc+1] as u16)};
        self.pc += 2;
        return op;
    }

    pub fn execute(&mut self, opcode: Opcode) {
        match opcode.opcode() {
            0x0 => {
                println!("OP code: {}", opcode.opcode());
            }
            
            0x1 => {
                println!("OP code: {}", opcode.opcode());
            }
            0x2 => {
                println!("OP code: {}", opcode.opcode());
            }
            0x3 => {
                println!("OP code: {}", opcode.opcode());
            }
            0x4 => {
                println!("OP code: {}", opcode.opcode());
            }
            0x5 => {
                println!("OP code: {}", opcode.opcode());
            }
            0x6 => {
                println!("OP code: {}", opcode.opcode());
            }
            0x7 => {
                println!("OP code: {}", opcode.opcode());
            }
            0x8 => {
                println!("OP code: {}", opcode.opcode());
            }
            0x9 => {
                println!("OP code: {}", opcode.opcode());
            }
            0xA => {
                println!("OP code: {}", opcode.opcode());
            }
            0xB => {
                println!("OP code: {}", opcode.opcode());
            }
            0xC => {
                println!("OP code: {}", opcode.opcode());
            }
            0xD => {
                println!("OP code: {}", opcode.opcode());
            }
            0xE => {
                println!("OP code: {}", opcode.opcode());
            }
            0xF => {
                println!("OP code: {}", opcode.opcode());
            }
            16_u8..=u8::MAX => {
                println!("You shouldn't reach this...");
            }
        }
    }

}

