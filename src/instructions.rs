enum LoadByteDestination {
    A, B, C, D, E, H, L, HLI, HLD, BC, DE, HL, A8, C_A8, A16
}

enum LoadByteSource {
    A, B, C, D, E, H, L, HLI, HLD, BC, DE, HL, A8, C_A8, A16, IMM8
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
            // TODO: HALT at 0x76
            _    => None // TODO: Implement a bajillion more opcodes
        }
    }
}

enum ArithmeticTarget {
    A, B, C, D, E, H, L,
}

