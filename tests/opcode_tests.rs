
#[cfg(test)]
mod tests{
    use rustchip8::Opcode;

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
}