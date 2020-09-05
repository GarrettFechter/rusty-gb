// Two letter registers, C_A8, A8, A16 are treated as addresses
enum LoadByteDestination {
    A, B, C, D, E, H, L, HLI, HLD, BC, DE, HL, A8, C_A8, A16
}

// Two letter registers, C_A8, A8, A16 are treated as addresses
enum LoadByteSource {
    A, B, C, D, E, H, L, HLI, HLD, BC, DE, HL, A8, C_A8, A16, IMM8
}

// Only A16 treated as address
enum LoadWordDestination {
    BC, DE, HL, SP, PUSH, AF, A16
}

enum LoadWordSource {
    BC, DE, HL, SP, POP, AF, IMM16, SP_IMM,
}

enum LoadType {
    Byte(LoadByteDestination, LoadByteSource),
    Word(LoadWordDestination, LoadWordSource), // 2 bytes
}

// HL is treated as address
enum ArithmeticTarget {
    B, C, D, E, H, L, HL, A, IMM8,
}

enum ArithmeticTarget16 {
    BC, DE, HL, SP, SP_IMM,
}

enum Instruction {
    ADD(ArithmeticTarget),
    SUB(ArithmeticTarget),
    AND(ArithmeticTarget),
    OR(ArithmeticTarget),
    ADC(ArithmeticTarget),
    SBC(ArithmeticTarget),
    XOR(ArithmeticTarget),
    CP(ArithmeticTarget),
    INC16(ArithmeticTarget16),
    ADD16(ArithmeticTarget16),
    DEC16(ArithmeticTarget16),
    LD(LoadType),
}

impl Instruction {
    fn from_byte(byte: u8, prefixed: bool) -> Option<Instruction> {
        if prefixed {
            Instruction::from_byte_prefixed(byte)
        } else {
            Instruction::from_byte_not_prefixed(byte)
        }
    }
    fn from_byte_not_prefixed(byte: u8) -> Option<Instruction> {
        match byte {
            0x13 => Some(Instruction::INC(IncDecTarget::DE)),

            // 8 bit loads

            0x02 => Some(Instruction::LD(LoadType::Byte(LoadByteDestination::BC, LoadByteSource::A))),
            0x12 => Some(Instruction::LD(LoadType::Byte(LoadByteDestination::DE, LoadByteSource::A))),
            0x22 => Some(Instruction::LD(LoadType::Byte(LoadByteDestination::HLI, LoadByteSource::A))),
            0x32 => Some(Instruction::LD(LoadType::Byte(LoadByteDestination::HLD, LoadByteSource::A))),

            0x06 => Some(Instruction::LD(LoadType::Byte(LoadByteDestination::B, LoadByteSource::IMM8))),
            0x16 => Some(Instruction::LD(LoadType::Byte(LoadByteDestination::D, LoadByteSource::IMM8))),
            0x26 => Some(Instruction::LD(LoadType::Byte(LoadByteDestination::H, LoadByteSource::IMM8))),
            0x36 => Some(Instruction::LD(LoadType::Byte(LoadByteDestination::HL, LoadByteSource::IMM8))),

            0x0A => Some(Instruction::LD(LoadType::Byte(LoadByteDestination::A, LoadByteSource::BC))),
            0x1A => Some(Instruction::LD(LoadType::Byte(LoadByteDestination::A, LoadByteSource::DE))),
            0x2A => Some(Instruction::LD(LoadType::Byte(LoadByteDestination::A, LoadByteSource::HLI))),
            0x3A => Some(Instruction::LD(LoadType::Byte(LoadByteDestination::A, LoadByteSource::HLD))),

            0x0E => Some(Instruction::LD(LoadType::Byte(LoadByteDestination::C, LoadByteSource::IMM8))),
            0x1E => Some(Instruction::LD(LoadType::Byte(LoadByteDestination::E, LoadByteSource::IMM8))),
            0x2E => Some(Instruction::LD(LoadType::Byte(LoadByteDestination::L, LoadByteSource::IMM8))),
            0x3E => Some(Instruction::LD(LoadType::Byte(LoadByteDestination::A, LoadByteSource::IMM8))),

            0x40 => Some(Instruction::LD(LoadType::Byte(LoadByteDestination::B, LoadByteSource::B))),
            0x41 => Some(Instruction::LD(LoadType::Byte(LoadByteDestination::B, LoadByteSource::C))),
            0x42 => Some(Instruction::LD(LoadType::Byte(LoadByteDestination::B, LoadByteSource::D))),
            0x43 => Some(Instruction::LD(LoadType::Byte(LoadByteDestination::B, LoadByteSource::E))),
            0x44 => Some(Instruction::LD(LoadType::Byte(LoadByteDestination::B, LoadByteSource::H))),
            0x45 => Some(Instruction::LD(LoadType::Byte(LoadByteDestination::B, LoadByteSource::L))),
            0x46 => Some(Instruction::LD(LoadType::Byte(LoadByteDestination::B, LoadByteSource::HL))),
            0x47 => Some(Instruction::LD(LoadType::Byte(LoadByteDestination::B, LoadByteSource::A))),
            0x48 => Some(Instruction::LD(LoadType::Byte(LoadByteDestination::C, LoadByteSource::B))),
            0x49 => Some(Instruction::LD(LoadType::Byte(LoadByteDestination::C, LoadByteSource::C))),
            0x4A => Some(Instruction::LD(LoadType::Byte(LoadByteDestination::C, LoadByteSource::D))),
            0x4B => Some(Instruction::LD(LoadType::Byte(LoadByteDestination::C, LoadByteSource::E))),
            0x4C => Some(Instruction::LD(LoadType::Byte(LoadByteDestination::C, LoadByteSource::H))),
            0x4D => Some(Instruction::LD(LoadType::Byte(LoadByteDestination::C, LoadByteSource::L))),
            0x4E => Some(Instruction::LD(LoadType::Byte(LoadByteDestination::C, LoadByteSource::HL))),
            0x4F => Some(Instruction::LD(LoadType::Byte(LoadByteDestination::C, LoadByteSource::A))),

            0x50 => Some(Instruction::LD(LoadType::Byte(LoadByteDestination::D, LoadByteSource::B))),
            0x51 => Some(Instruction::LD(LoadType::Byte(LoadByteDestination::D, LoadByteSource::C))),
            0x52 => Some(Instruction::LD(LoadType::Byte(LoadByteDestination::D, LoadByteSource::D))),
            0x53 => Some(Instruction::LD(LoadType::Byte(LoadByteDestination::D, LoadByteSource::E))),
            0x54 => Some(Instruction::LD(LoadType::Byte(LoadByteDestination::D, LoadByteSource::H))),
            0x55 => Some(Instruction::LD(LoadType::Byte(LoadByteDestination::D, LoadByteSource::L))),
            0x56 => Some(Instruction::LD(LoadType::Byte(LoadByteDestination::D, LoadByteSource::HL))),
            0x57 => Some(Instruction::LD(LoadType::Byte(LoadByteDestination::D, LoadByteSource::A))),
            0x58 => Some(Instruction::LD(LoadType::Byte(LoadByteDestination::E, LoadByteSource::B))),
            0x59 => Some(Instruction::LD(LoadType::Byte(LoadByteDestination::E, LoadByteSource::C))),
            0x5A => Some(Instruction::LD(LoadType::Byte(LoadByteDestination::E, LoadByteSource::D))),
            0x5B => Some(Instruction::LD(LoadType::Byte(LoadByteDestination::E, LoadByteSource::E))),
            0x5C => Some(Instruction::LD(LoadType::Byte(LoadByteDestination::E, LoadByteSource::H))),
            0x5D => Some(Instruction::LD(LoadType::Byte(LoadByteDestination::E, LoadByteSource::L))),
            0x5E => Some(Instruction::LD(LoadType::Byte(LoadByteDestination::E, LoadByteSource::HL))),
            0x5F => Some(Instruction::LD(LoadType::Byte(LoadByteDestination::E, LoadByteSource::A))),

            0x60 => Some(Instruction::LD(LoadType::Byte(LoadByteDestination::H, LoadByteSource::B))),
            0x61 => Some(Instruction::LD(LoadType::Byte(LoadByteDestination::H, LoadByteSource::C))),
            0x62 => Some(Instruction::LD(LoadType::Byte(LoadByteDestination::H, LoadByteSource::D))),
            0x63 => Some(Instruction::LD(LoadType::Byte(LoadByteDestination::H, LoadByteSource::E))),
            0x64 => Some(Instruction::LD(LoadType::Byte(LoadByteDestination::H, LoadByteSource::H))),
            0x65 => Some(Instruction::LD(LoadType::Byte(LoadByteDestination::H, LoadByteSource::L))),
            0x66 => Some(Instruction::LD(LoadType::Byte(LoadByteDestination::H, LoadByteSource::HL))),
            0x67 => Some(Instruction::LD(LoadType::Byte(LoadByteDestination::H, LoadByteSource::A))),
            0x68 => Some(Instruction::LD(LoadType::Byte(LoadByteDestination::L, LoadByteSource::B))),
            0x69 => Some(Instruction::LD(LoadType::Byte(LoadByteDestination::L, LoadByteSource::C))),
            0x6A => Some(Instruction::LD(LoadType::Byte(LoadByteDestination::L, LoadByteSource::D))),
            0x6B => Some(Instruction::LD(LoadType::Byte(LoadByteDestination::L, LoadByteSource::E))),
            0x6C => Some(Instruction::LD(LoadType::Byte(LoadByteDestination::L, LoadByteSource::H))),
            0x6D => Some(Instruction::LD(LoadType::Byte(LoadByteDestination::L, LoadByteSource::L))),
            0x6E => Some(Instruction::LD(LoadType::Byte(LoadByteDestination::L, LoadByteSource::HL))),
            0x6F => Some(Instruction::LD(LoadType::Byte(LoadByteDestination::L, LoadByteSource::A))),

            0x70 => Some(Instruction::LD(LoadType::Byte(LoadByteDestination::HL, LoadByteSource::B))),
            0x71 => Some(Instruction::LD(LoadType::Byte(LoadByteDestination::HL, LoadByteSource::C))),
            0x72 => Some(Instruction::LD(LoadType::Byte(LoadByteDestination::HL, LoadByteSource::D))),
            0x73 => Some(Instruction::LD(LoadType::Byte(LoadByteDestination::HL, LoadByteSource::E))),
            0x74 => Some(Instruction::LD(LoadType::Byte(LoadByteDestination::HL, LoadByteSource::H))),
            0x75 => Some(Instruction::LD(LoadType::Byte(LoadByteDestination::HL, LoadByteSource::L))),
            0x77 => Some(Instruction::LD(LoadType::Byte(LoadByteDestination::HL, LoadByteSource::A))),
            0x78 => Some(Instruction::LD(LoadType::Byte(LoadByteDestination::A, LoadByteSource::B))),
            0x79 => Some(Instruction::LD(LoadType::Byte(LoadByteDestination::A, LoadByteSource::C))),
            0x7A => Some(Instruction::LD(LoadType::Byte(LoadByteDestination::A, LoadByteSource::D))),
            0x7B => Some(Instruction::LD(LoadType::Byte(LoadByteDestination::A, LoadByteSource::E))),
            0x7C => Some(Instruction::LD(LoadType::Byte(LoadByteDestination::A, LoadByteSource::H))),
            0x7D => Some(Instruction::LD(LoadType::Byte(LoadByteDestination::A, LoadByteSource::L))),
            0x7E => Some(Instruction::LD(LoadType::Byte(LoadByteDestination::A, LoadByteSource::HL))),
            0x7F => Some(Instruction::LD(LoadType::Byte(LoadByteDestination::A, LoadByteSource::A))),

            0xE0 => Some(Instruction::LD(LoadType::Byte(LoadByteDestination::A8, LoadByteSource::A))),
            0xF0 => Some(Instruction::LD(LoadType::Byte(LoadByteDestination::A, LoadByteSource::A8))),

            0xE2 => Some(Instruction::LD(LoadType::Byte(LoadByteDestination::C_A8, LoadByteSource::A))),
            0xF2 => Some(Instruction::LD(LoadType::Byte(LoadByteDestination::A, LoadByteSource::C_A8))),

            0xEA => Some(Instruction::LD(LoadType::Byte(LoadByteDestination::A16, LoadByteSource::A))),
            0xFA => Some(Instruction::LD(LoadType::Byte(LoadByteDestination::A, LoadByteSource::A16))),

            // 16 bit loads

            0x01 => Some(Instruction::LD(LoadType::Word(LoadWordDestination::BC, LoadWordSource::IMM16))),
            0x21 => Some(Instruction::LD(LoadType::Word(LoadWordDestination::DE, LoadWordSource::IMM16))),
            0x31 => Some(Instruction::LD(LoadType::Word(LoadWordDestination::HL, LoadWordSource::IMM16))),
            0x41 => Some(Instruction::LD(LoadType::Word(LoadWordDestination::SP, LoadWordSource::IMM16))),

            0xC1 => Some(Instruction::LD(LoadType::Word(LoadWordDestination::BC, LoadWordSource::POP))),
            0xD1 => Some(Instruction::LD(LoadType::Word(LoadWordDestination::DE, LoadWordSource::POP))),
            0xE1 => Some(Instruction::LD(LoadType::Word(LoadWordDestination::HL, LoadWordSource::POP))),
            0xF1 => Some(Instruction::LD(LoadType::Word(LoadWordDestination::AF, LoadWordSource::POP))),

            0x08 => Some(Instruction::LD(LoadType::Word(LoadWordDestination::A16, LoadWordSource::SP))),
            0xF8 => Some(Instruction::LD(LoadType::Word(LoadWordDestination::HL, LoadWordSource::SP_IMM))),
            0xF9 => Some(Instruction::LD(LoadType::Word(LoadWordDestination::SP, LoadWordSource::HL))),

            // 8 bit arithmetic

            0x04 => Some(Instruction::INC(ArithmeticTarget::B));
            0x14 => Some(Instruction::INC(ArithmeticTarget::D));
            0x24 => Some(Instruction::INC(ArithmeticTarget::H));
            0x34 => Some(Instruction::INC(ArithmeticTarget::HL));

            0x05 => Some(Instruction::DEC(ArithmeticTarget::B));
            0x15 => Some(Instruction::DEC(ArithmeticTarget::D));
            0x25 => Some(Instruction::DEC(ArithmeticTarget::H));
            0x35 => Some(Instruction::DEC(ArithmeticTarget::HL));

            0x0C => Some(Instruction::INC(ArithmeticTarget::C));
            0x1C => Some(Instruction::INC(ArithmeticTarget::E));
            0x2C => Some(Instruction::INC(ArithmeticTarget::L));
            0x3C => Some(Instruction::INC(ArithmeticTarget::A));

            0x0D => Some(Instruction::DEC(ArithmeticTarget::C));
            0x1D => Some(Instruction::DEC(ArithmeticTarget::E));
            0x2D => Some(Instruction::DEC(ArithmeticTarget::L));
            0x3D => Some(Instruction::DEC(ArithmeticTarget::A));

            0x80 => Some(Instruction::ADD(ArithmeticTarget::B));
            0x81 => Some(Instruction::ADD(ArithmeticTarget::C));
            0x82 => Some(Instruction::ADD(ArithmeticTarget::D));
            0x83 => Some(Instruction::ADD(ArithmeticTarget::E));
            0x84 => Some(Instruction::ADD(ArithmeticTarget::H));
            0x85 => Some(Instruction::ADD(ArithmeticTarget::L));
            0x86 => Some(Instruction::ADD(ArithmeticTarget::HL));
            0x87 => Some(Instruction::ADD(ArithmeticTarget::A));

            0x88 => Some(Instruction::ADC(ArithmeticTarget::B));
            0x89 => Some(Instruction::ADC(ArithmeticTarget::C));
            0x8A => Some(Instruction::ADC(ArithmeticTarget::D));
            0x8B => Some(Instruction::ADC(ArithmeticTarget::E));
            0x8C => Some(Instruction::ADC(ArithmeticTarget::H));
            0x8D => Some(Instruction::ADC(ArithmeticTarget::L));
            0x8E => Some(Instruction::ADC(ArithmeticTarget::HL));
            0x8F => Some(Instruction::ADC(ArithmeticTarget::A));

            0x90 => Some(Instruction::SUB(ArithmeticTarget::B));
            0x91 => Some(Instruction::SUB(ArithmeticTarget::C));
            0x92 => Some(Instruction::SUB(ArithmeticTarget::D));
            0x93 => Some(Instruction::SUB(ArithmeticTarget::E));
            0x94 => Some(Instruction::SUB(ArithmeticTarget::H));
            0x95 => Some(Instruction::SUB(ArithmeticTarget::L));
            0x96 => Some(Instruction::SUB(ArithmeticTarget::HL));
            0x97 => Some(Instruction::SUB(ArithmeticTarget::A));

            0x98 => Some(Instruction::SBC(ArithmeticTarget::B));
            0x99 => Some(Instruction::SBC(ArithmeticTarget::C));
            0x9A => Some(Instruction::SBC(ArithmeticTarget::D));
            0x9B => Some(Instruction::SBC(ArithmeticTarget::E));
            0x9C => Some(Instruction::SBC(ArithmeticTarget::H));
            0x9D => Some(Instruction::SBC(ArithmeticTarget::L));
            0x9E => Some(Instruction::SBC(ArithmeticTarget::HL));
            0x9F => Some(Instruction::SBC(ArithmeticTarget::A));

            0xA0 => Some(Instruction::AND(ArithmeticTarget::B));
            0xA1 => Some(Instruction::AND(ArithmeticTarget::C));
            0xA2 => Some(Instruction::AND(ArithmeticTarget::D));
            0xA3 => Some(Instruction::AND(ArithmeticTarget::E));
            0xA4 => Some(Instruction::AND(ArithmeticTarget::H));
            0xA5 => Some(Instruction::AND(ArithmeticTarget::L));
            0xA6 => Some(Instruction::AND(ArithmeticTarget::HL));
            0xA7 => Some(Instruction::AND(ArithmeticTarget::A));

            0xA8 => Some(Instruction::XOR(ArithmeticTarget::B));
            0xA9 => Some(Instruction::XOR(ArithmeticTarget::C));
            0xAA => Some(Instruction::XOR(ArithmeticTarget::D));
            0xAB => Some(Instruction::XOR(ArithmeticTarget::E));
            0xAC => Some(Instruction::XOR(ArithmeticTarget::H));
            0xAD => Some(Instruction::XOR(ArithmeticTarget::L));
            0xAE => Some(Instruction::XOR(ArithmeticTarget::HL));
            0xAF => Some(Instruction::XOR(ArithmeticTarget::A));

            0xB0 => Some(Instruction::OR(ArithmeticTarget::B));
            0xB1 => Some(Instruction::OR(ArithmeticTarget::C));
            0xB2 => Some(Instruction::OR(ArithmeticTarget::D));
            0xB3 => Some(Instruction::OR(ArithmeticTarget::E));
            0xB4 => Some(Instruction::OR(ArithmeticTarget::H));
            0xB5 => Some(Instruction::OR(ArithmeticTarget::L));
            0xB6 => Some(Instruction::OR(ArithmeticTarget::HL));
            0xB7 => Some(Instruction::OR(ArithmeticTarget::A));

            0xB8 => Some(Instruction::CP(ArithmeticTarget::B));
            0xB9 => Some(Instruction::CP(ArithmeticTarget::C));
            0xBA => Some(Instruction::CP(ArithmeticTarget::D));
            0xBB => Some(Instruction::CP(ArithmeticTarget::E));
            0xBC => Some(Instruction::CP(ArithmeticTarget::H));
            0xBD => Some(Instruction::CP(ArithmeticTarget::L));
            0xBE => Some(Instruction::CP(ArithmeticTarget::HL));
            0xBF => Some(Instruction::CP(ArithmeticTarget::A));

            0xC6 => Some(Instruction::ADD(ArithmeticTarget::IMM8));
            0xD6 => Some(Instruction::SUB(ArithmeticTarget::IMM8));
            0xE6 => Some(Instruction::AND(ArithmeticTarget::IMM8));
            0xF6 => Some(Instruction::OR(ArithmeticTarget::IMM8));

            0xCE => Some(Instruction::ADC(ArithmeticTarget::IMM8));
            0xDE => Some(Instruction::SBC(ArithmeticTarget::IMM8));
            0xEE => Some(Instruction::XOR(ArithmeticTarget::IMM8));
            0xFE => Some(Instruction::CP(ArithmeticTarget::IMM8));

            // 16 bit arithmetic

            0x03 => Some(Instruction::INC16(ArithmeticTarget16::BC));
            0x13 => Some(Instruction::INC16(ArithmeticTarget16::DE));
            0x23 => Some(Instruction::INC16(ArithmeticTarget16::HL));
            0x33 => Some(Instruction::INC16(ArithmeticTarget16::SP));

            0x09 => Some(Instruction::ADD16(ArithmeticTarget16::BC));
            0x19 => Some(Instruction::ADD16(ArithmeticTarget16::DE));
            0x29 => Some(Instruction::ADD16(ArithmeticTarget16::HL));
            0x39 => Some(Instruction::ADD16(ArithmeticTarget16::SP));

            0x0B => Some(Instruction::ADD16(ArithmeticTarget16::BC));
            0x1B => Some(Instruction::ADD16(ArithmeticTarget16::DE));
            0x2B => Some(Instruction::ADD16(ArithmeticTarget16::HL));
            0x3B => Some(Instruction::ADD16(ArithmeticTarget16::SP));

            0xE8 => Some(Instruction::ADD16(ArithmeticTarget16::SP_IMM));

            // TODO: HALT at 0x76
            _    => None // TODO: Implement no-prefix opcodes
        }
    }
    fn from_byte_prefixed(byte: u8) -> Option<Instruction> {
        match byte {
            _ => None // TODO: Implement prefixed opcodes
        }
    }
}
