
mod chip8;
pub use crate::chip8::CPU;
pub use crate::chip8::Opcode;


fn main() {
    println!("Hello, world!");
    let mut cpu = CPU::new();
    println!("Fetch: {}", cpu.fetch().op);
    println!("Fetch: {}", cpu.fetch().x());
}
