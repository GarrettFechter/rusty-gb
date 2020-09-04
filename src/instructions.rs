enum LoadByteDestination {
    A, B, C, D, E, H, L, HLI, HLD, BC, DE, A8, A16
}

enum LoadByteSource {
    A, B, C, D, E, H, L, HLI, HLD, BC, DE, A8, A16, IMM8
}

enum LoadType {
    Byte(LoadDestination, LoadSource),
}

enum Instruction {
    ADD(ArithmeticTarget),
    LD(LoadType),
}

impl Instruction {
    fn from_byte(byte: u8) -> Option<Instruction> {
        match byte {
            0x02 => Some(Instruction::INC(IncDecTarget::BC)),
            0x13 => Some(Instruction::INC(IncDecTarget::DE)),
            _    => None // TODO: Implement a bajillion more opcodes
        }
    }
}

enum ArithmeticTarget {
    A, B, C, D, E, H, L,
}

