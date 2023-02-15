
#[cfg(test)]
mod tests{
    use rustchip8::Opcode;
    use rustchip8::CPU;

    #[test]
    pub fn test_opcode_fetch() {
        let op = Opcode{op: 0x1234};
        assert_eq!(op.opcode(), 0x1);
        assert_eq!(op.x(), 0x2);
        assert_eq!(op.y(), 0x3);
        assert_eq!(op.n(), 0x4);
        assert_eq!(op.nn(), 0x34);
        assert_eq!(op.nnn(), 0x234);
    }

    #[test]
    pub fn test_overflow_add() {
        let cpu = CPU::new();
        assert_eq!(cpu.overflow_add(255, 100), 99);
        assert_eq!(cpu.overflow_add(255, 10), 9);
        assert_eq!(cpu.overflow_add(200, 100), 44);
        assert_eq!(cpu.overflow_add(123, 123), 246);
    }

    #[test]
    pub fn test_overflow_subtract() {
        let cpu = CPU::new();
        assert_eq!(cpu.overflow_subtract(100, 255), 101);
        assert_eq!(cpu.overflow_subtract(10, 255), 11);
        assert_eq!(cpu.overflow_subtract(100, 200), 156);
        assert_eq!(cpu.overflow_subtract(123, 123), 0);
    }

}