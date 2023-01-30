pub fn test() {
    println!("Hello");
}

pub struct Peripherals {
    pub test: u8,
}

impl Peripherals {

    pub fn new() -> Peripherals {
        let peripherals = Peripherals{test: 1};
        peripherals
    }
}