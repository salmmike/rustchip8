use rustchip8::CPU;

fn main() {
    println!("Hello, world!");
    let mut cpu = CPU::new();
    println!("Fetch: {}", cpu.fetch().op);
    println!("Fetch: {}", cpu.fetch().x());
}
