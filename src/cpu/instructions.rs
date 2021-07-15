// Two letter registers, CA8, A8, A16 are treated as addresses
pub enum LoadByteDestination {
    A, B, C, D, E, H, L, HLI, HLD, BC, DE, HL, A8, CA8, A16,
}

// Two letter registers, CA8, A8, A16 are treated as addresses
pub enum LoadByteSource {
    A, B, C, D, E, H, L, HLI, HLD, BC, DE, HL, A8, CA8, A16, IMM8,
}

// Only A16 treated as address
pub enum LoadWordDestination {
    BC, DE, HL, SP, PUSH, AF, A16,
}

pub enum LoadWordSource {
    BC, DE, HL, SP, POP, AF, IMM16, SPIMM,
}

pub enum LoadType {
    Byte(LoadByteDestination, LoadByteSource),
    Word(LoadWordDestination, LoadWordSource), // 2 bytes
}

// HL is treated as address
pub enum ArithmeticTarget {
    B, C, D, E, H, L, HL, A, IMM8,
}

pub enum ArithmeticTarget16 {
    BC, DE, HL, SP, SPIMM,
}

pub enum ControlCondition {
    NZ, NC, Z, C, NONE, NONEEI,
}

pub enum RstValue {
    H00, H10, H20, H30, H08, H18, H28, H38,
}

pub enum JumpAddr {
    IMM16, HL, REL,
}

pub enum Instruction {
    LD(LoadType),
    SUB(ArithmeticTarget),
    AND(ArithmeticTarget),
    OR(ArithmeticTarget),
    ADC(ArithmeticTarget),
    SBC(ArithmeticTarget),
    XOR(ArithmeticTarget),
    CP(ArithmeticTarget),
    INC(ArithmeticTarget),
    INC16(ArithmeticTarget16),
    ADD(ArithmeticTarget),
    ADD16(ArithmeticTarget16),
    DEC(ArithmeticTarget),
    DEC16(ArithmeticTarget16),
    JP(ControlCondition, JumpAddr),
    RET(ControlCondition),
    CALL(ControlCondition),
    RST(RstValue),
    NOP,
    STOP,
    HALT,
    DI,
    EI,
    CPL,
    CCF,
    DAA,
    SCF,
}

impl Instruction {
    // Return (cycles used, Option<Instruction>)
    pub fn from_byte(byte: u8, prefixed: bool) -> (u16, Option<Instruction>) {
        if prefixed {
            Instruction::from_byte_prefixed(byte)
        } else {
            Instruction::from_byte_not_prefixed(byte)
        }
    }

    // Return (cycles used, Option<Instruction>)
    fn from_byte_not_prefixed(byte: u8) -> (u16, Option<Instruction>) {
        match byte {

            // 8 bit loads
            0x02 => (8, Some(Instruction::LD(LoadType::Byte(LoadByteDestination::BC, LoadByteSource::A)))),
            0x12 => (8, Some(Instruction::LD(LoadType::Byte(LoadByteDestination::DE, LoadByteSource::A)))),
            0x22 => (8, Some(Instruction::LD(LoadType::Byte(LoadByteDestination::HLI, LoadByteSource::A)))),
            0x32 => (8, Some(Instruction::LD(LoadType::Byte(LoadByteDestination::HLD, LoadByteSource::A)))),

            0x06 => (8, Some(Instruction::LD(LoadType::Byte(LoadByteDestination::B, LoadByteSource::IMM8)))),
            0x16 => (8, Some(Instruction::LD(LoadType::Byte(LoadByteDestination::D, LoadByteSource::IMM8)))),
            0x26 => (8, Some(Instruction::LD(LoadType::Byte(LoadByteDestination::H, LoadByteSource::IMM8)))),
            0x36 => (12, Some(Instruction::LD(LoadType::Byte(LoadByteDestination::HL, LoadByteSource::IMM8)))),

            0x0A => (8, Some(Instruction::LD(LoadType::Byte(LoadByteDestination::A, LoadByteSource::BC)))),
            0x1A => (8, Some(Instruction::LD(LoadType::Byte(LoadByteDestination::A, LoadByteSource::DE)))),
            0x2A => (8, Some(Instruction::LD(LoadType::Byte(LoadByteDestination::A, LoadByteSource::HLI)))),
            0x3A => (8, Some(Instruction::LD(LoadType::Byte(LoadByteDestination::A, LoadByteSource::HLD)))),

            0x0E => (8, Some(Instruction::LD(LoadType::Byte(LoadByteDestination::C, LoadByteSource::IMM8)))),
            0x1E => (8, Some(Instruction::LD(LoadType::Byte(LoadByteDestination::E, LoadByteSource::IMM8)))),
            0x2E => (8, Some(Instruction::LD(LoadType::Byte(LoadByteDestination::L, LoadByteSource::IMM8)))),
            0x3E => (8, Some(Instruction::LD(LoadType::Byte(LoadByteDestination::A, LoadByteSource::IMM8)))),

            0x40 => (4, Some(Instruction::LD(LoadType::Byte(LoadByteDestination::B, LoadByteSource::B)))),
            0x41 => (4, Some(Instruction::LD(LoadType::Byte(LoadByteDestination::B, LoadByteSource::C)))),
            0x42 => (4, Some(Instruction::LD(LoadType::Byte(LoadByteDestination::B, LoadByteSource::D)))),
            0x43 => (4, Some(Instruction::LD(LoadType::Byte(LoadByteDestination::B, LoadByteSource::E)))),
            0x44 => (4, Some(Instruction::LD(LoadType::Byte(LoadByteDestination::B, LoadByteSource::H)))),
            0x45 => (4, Some(Instruction::LD(LoadType::Byte(LoadByteDestination::B, LoadByteSource::L)))),
            0x46 => (8, Some(Instruction::LD(LoadType::Byte(LoadByteDestination::B, LoadByteSource::HL)))),
            0x47 => (4, Some(Instruction::LD(LoadType::Byte(LoadByteDestination::B, LoadByteSource::A)))),
            0x48 => (4, Some(Instruction::LD(LoadType::Byte(LoadByteDestination::C, LoadByteSource::B)))),
            0x49 => (4, Some(Instruction::LD(LoadType::Byte(LoadByteDestination::C, LoadByteSource::C)))),
            0x4A => (4, Some(Instruction::LD(LoadType::Byte(LoadByteDestination::C, LoadByteSource::D)))),
            0x4B => (4, Some(Instruction::LD(LoadType::Byte(LoadByteDestination::C, LoadByteSource::E)))),
            0x4C => (4, Some(Instruction::LD(LoadType::Byte(LoadByteDestination::C, LoadByteSource::H)))),
            0x4D => (4, Some(Instruction::LD(LoadType::Byte(LoadByteDestination::C, LoadByteSource::L)))),
            0x4E => (8, Some(Instruction::LD(LoadType::Byte(LoadByteDestination::C, LoadByteSource::HL)))),
            0x4F => (4, Some(Instruction::LD(LoadType::Byte(LoadByteDestination::C, LoadByteSource::A)))),

            0x50 => (4, Some(Instruction::LD(LoadType::Byte(LoadByteDestination::D, LoadByteSource::B)))),
            0x51 => (4, Some(Instruction::LD(LoadType::Byte(LoadByteDestination::D, LoadByteSource::C)))),
            0x52 => (4, Some(Instruction::LD(LoadType::Byte(LoadByteDestination::D, LoadByteSource::D)))),
            0x53 => (4, Some(Instruction::LD(LoadType::Byte(LoadByteDestination::D, LoadByteSource::E)))),
            0x54 => (4, Some(Instruction::LD(LoadType::Byte(LoadByteDestination::D, LoadByteSource::H)))),
            0x55 => (4, Some(Instruction::LD(LoadType::Byte(LoadByteDestination::D, LoadByteSource::L)))),
            0x56 => (8, Some(Instruction::LD(LoadType::Byte(LoadByteDestination::D, LoadByteSource::HL)))),
            0x57 => (4, Some(Instruction::LD(LoadType::Byte(LoadByteDestination::D, LoadByteSource::A)))),
            0x58 => (4, Some(Instruction::LD(LoadType::Byte(LoadByteDestination::E, LoadByteSource::B)))),
            0x59 => (4, Some(Instruction::LD(LoadType::Byte(LoadByteDestination::E, LoadByteSource::C)))),
            0x5A => (4, Some(Instruction::LD(LoadType::Byte(LoadByteDestination::E, LoadByteSource::D)))),
            0x5B => (4, Some(Instruction::LD(LoadType::Byte(LoadByteDestination::E, LoadByteSource::E)))),
            0x5C => (4, Some(Instruction::LD(LoadType::Byte(LoadByteDestination::E, LoadByteSource::H)))),
            0x5D => (4, Some(Instruction::LD(LoadType::Byte(LoadByteDestination::E, LoadByteSource::L)))),
            0x5E => (8, Some(Instruction::LD(LoadType::Byte(LoadByteDestination::E, LoadByteSource::HL)))),
            0x5F => (4, Some(Instruction::LD(LoadType::Byte(LoadByteDestination::E, LoadByteSource::A)))),

            0x60 => (4, Some(Instruction::LD(LoadType::Byte(LoadByteDestination::H, LoadByteSource::B)))),
            0x61 => (4, Some(Instruction::LD(LoadType::Byte(LoadByteDestination::H, LoadByteSource::C)))),
            0x62 => (4, Some(Instruction::LD(LoadType::Byte(LoadByteDestination::H, LoadByteSource::D)))),
            0x63 => (4, Some(Instruction::LD(LoadType::Byte(LoadByteDestination::H, LoadByteSource::E)))),
            0x64 => (4, Some(Instruction::LD(LoadType::Byte(LoadByteDestination::H, LoadByteSource::H)))),
            0x65 => (4, Some(Instruction::LD(LoadType::Byte(LoadByteDestination::H, LoadByteSource::L)))),
            0x66 => (8, Some(Instruction::LD(LoadType::Byte(LoadByteDestination::H, LoadByteSource::HL)))),
            0x67 => (4, Some(Instruction::LD(LoadType::Byte(LoadByteDestination::H, LoadByteSource::A)))),
            0x68 => (4, Some(Instruction::LD(LoadType::Byte(LoadByteDestination::L, LoadByteSource::B)))),
            0x69 => (4, Some(Instruction::LD(LoadType::Byte(LoadByteDestination::L, LoadByteSource::C)))),
            0x6A => (4, Some(Instruction::LD(LoadType::Byte(LoadByteDestination::L, LoadByteSource::D)))),
            0x6B => (4, Some(Instruction::LD(LoadType::Byte(LoadByteDestination::L, LoadByteSource::E)))),
            0x6C => (4, Some(Instruction::LD(LoadType::Byte(LoadByteDestination::L, LoadByteSource::H)))),
            0x6D => (4, Some(Instruction::LD(LoadType::Byte(LoadByteDestination::L, LoadByteSource::L)))),
            0x6E => (8, Some(Instruction::LD(LoadType::Byte(LoadByteDestination::L, LoadByteSource::HL)))),
            0x6F => (4, Some(Instruction::LD(LoadType::Byte(LoadByteDestination::L, LoadByteSource::A)))),

            0x70 => (8, Some(Instruction::LD(LoadType::Byte(LoadByteDestination::HL, LoadByteSource::B)))),
            0x71 => (8, Some(Instruction::LD(LoadType::Byte(LoadByteDestination::HL, LoadByteSource::C)))),
            0x72 => (8, Some(Instruction::LD(LoadType::Byte(LoadByteDestination::HL, LoadByteSource::D)))),
            0x73 => (8, Some(Instruction::LD(LoadType::Byte(LoadByteDestination::HL, LoadByteSource::E)))),
            0x74 => (8, Some(Instruction::LD(LoadType::Byte(LoadByteDestination::HL, LoadByteSource::H)))),
            0x75 => (8, Some(Instruction::LD(LoadType::Byte(LoadByteDestination::HL, LoadByteSource::L)))),
            0x77 => (8, Some(Instruction::LD(LoadType::Byte(LoadByteDestination::HL, LoadByteSource::A)))),
            0x78 => (4, Some(Instruction::LD(LoadType::Byte(LoadByteDestination::A, LoadByteSource::B)))),
            0x79 => (4, Some(Instruction::LD(LoadType::Byte(LoadByteDestination::A, LoadByteSource::C)))),
            0x7A => (4, Some(Instruction::LD(LoadType::Byte(LoadByteDestination::A, LoadByteSource::D)))),
            0x7B => (4, Some(Instruction::LD(LoadType::Byte(LoadByteDestination::A, LoadByteSource::E)))),
            0x7C => (4, Some(Instruction::LD(LoadType::Byte(LoadByteDestination::A, LoadByteSource::H)))),
            0x7D => (4, Some(Instruction::LD(LoadType::Byte(LoadByteDestination::A, LoadByteSource::L)))),
            0x7E => (8, Some(Instruction::LD(LoadType::Byte(LoadByteDestination::A, LoadByteSource::HL)))),
            0x7F => (4, Some(Instruction::LD(LoadType::Byte(LoadByteDestination::A, LoadByteSource::A)))),

            0xE0 => (12, Some(Instruction::LD(LoadType::Byte(LoadByteDestination::A8, LoadByteSource::A)))),
            0xF0 => (12, Some(Instruction::LD(LoadType::Byte(LoadByteDestination::A, LoadByteSource::A8)))),

            0xE2 => (8, Some(Instruction::LD(LoadType::Byte(LoadByteDestination::CA8, LoadByteSource::A)))),
            0xF2 => (8, Some(Instruction::LD(LoadType::Byte(LoadByteDestination::A, LoadByteSource::CA8)))),

            0xEA => (16, Some(Instruction::LD(LoadType::Byte(LoadByteDestination::A16, LoadByteSource::A)))),
            0xFA => (16, Some(Instruction::LD(LoadType::Byte(LoadByteDestination::A, LoadByteSource::A16)))),

            // 16 bit loads

            0x01 => (12, Some(Instruction::LD(LoadType::Word(LoadWordDestination::BC, LoadWordSource::IMM16)))),
            0x11 => (12, Some(Instruction::LD(LoadType::Word(LoadWordDestination::DE, LoadWordSource::IMM16)))),
            0x21 => (12, Some(Instruction::LD(LoadType::Word(LoadWordDestination::HL, LoadWordSource::IMM16)))),
            0x31 => (12, Some(Instruction::LD(LoadType::Word(LoadWordDestination::SP, LoadWordSource::IMM16)))),

            0xC1 => (12, Some(Instruction::LD(LoadType::Word(LoadWordDestination::BC, LoadWordSource::POP)))),
            0xD1 => (12, Some(Instruction::LD(LoadType::Word(LoadWordDestination::DE, LoadWordSource::POP)))),
            0xE1 => (12, Some(Instruction::LD(LoadType::Word(LoadWordDestination::HL, LoadWordSource::POP)))),
            0xF1 => (12, Some(Instruction::LD(LoadType::Word(LoadWordDestination::AF, LoadWordSource::POP)))),

            0xC5 => (16, Some(Instruction::LD(LoadType::Word(LoadWordDestination::PUSH, LoadWordSource::BC)))),
            0xD5 => (16, Some(Instruction::LD(LoadType::Word(LoadWordDestination::PUSH, LoadWordSource::DE)))),
            0xE5 => (16, Some(Instruction::LD(LoadType::Word(LoadWordDestination::PUSH, LoadWordSource::HL)))),
            0xF5 => (16, Some(Instruction::LD(LoadType::Word(LoadWordDestination::PUSH, LoadWordSource::AF)))),

            0x08 => (20, Some(Instruction::LD(LoadType::Word(LoadWordDestination::A16, LoadWordSource::SP)))),
            0xF8 => (12, Some(Instruction::LD(LoadType::Word(LoadWordDestination::HL, LoadWordSource::SPIMM)))),
            0xF9 => (8, Some(Instruction::LD(LoadType::Word(LoadWordDestination::SP, LoadWordSource::HL)))),

            // 8 bit arithmetic

            0x04 => (4, Some(Instruction::INC(ArithmeticTarget::B))),
            0x14 => (4, Some(Instruction::INC(ArithmeticTarget::D))),
            0x24 => (4, Some(Instruction::INC(ArithmeticTarget::H))),
            0x34 => (12, Some(Instruction::INC(ArithmeticTarget::HL))),

            0x05 => (4, Some(Instruction::DEC(ArithmeticTarget::B))),
            0x15 => (4, Some(Instruction::DEC(ArithmeticTarget::D))),
            0x25 => (4, Some(Instruction::DEC(ArithmeticTarget::H))),
            0x35 => (12, Some(Instruction::DEC(ArithmeticTarget::HL))),

            0x0C => (4, Some(Instruction::INC(ArithmeticTarget::C))),
            0x1C => (4, Some(Instruction::INC(ArithmeticTarget::E))),
            0x2C => (4, Some(Instruction::INC(ArithmeticTarget::L))),
            0x3C => (4, Some(Instruction::INC(ArithmeticTarget::A))),

            0x0D => (4, Some(Instruction::DEC(ArithmeticTarget::C))),
            0x1D => (4, Some(Instruction::DEC(ArithmeticTarget::E))),
            0x2D => (4, Some(Instruction::DEC(ArithmeticTarget::L))),
            0x3D => (4, Some(Instruction::DEC(ArithmeticTarget::A))),

            0x80 => (4, Some(Instruction::ADD(ArithmeticTarget::B))),
            0x81 => (4, Some(Instruction::ADD(ArithmeticTarget::C))),
            0x82 => (4, Some(Instruction::ADD(ArithmeticTarget::D))),
            0x83 => (4, Some(Instruction::ADD(ArithmeticTarget::E))),
            0x84 => (4, Some(Instruction::ADD(ArithmeticTarget::H))),
            0x85 => (4, Some(Instruction::ADD(ArithmeticTarget::L))),
            0x86 => (8, Some(Instruction::ADD(ArithmeticTarget::HL))),
            0x87 => (4, Some(Instruction::ADD(ArithmeticTarget::A))),

            0x88 => (4, Some(Instruction::ADC(ArithmeticTarget::B))),
            0x89 => (4, Some(Instruction::ADC(ArithmeticTarget::C))),
            0x8A => (4, Some(Instruction::ADC(ArithmeticTarget::D))),
            0x8B => (4, Some(Instruction::ADC(ArithmeticTarget::E))),
            0x8C => (4, Some(Instruction::ADC(ArithmeticTarget::H))),
            0x8D => (4, Some(Instruction::ADC(ArithmeticTarget::L))),
            0x8E => (8, Some(Instruction::ADC(ArithmeticTarget::HL))),
            0x8F => (4, Some(Instruction::ADC(ArithmeticTarget::A))),

            0x90 => (4, Some(Instruction::SUB(ArithmeticTarget::B))),
            0x91 => (4, Some(Instruction::SUB(ArithmeticTarget::C))),
            0x92 => (4, Some(Instruction::SUB(ArithmeticTarget::D))),
            0x93 => (4, Some(Instruction::SUB(ArithmeticTarget::E))),
            0x94 => (4, Some(Instruction::SUB(ArithmeticTarget::H))),
            0x95 => (4, Some(Instruction::SUB(ArithmeticTarget::L))),
            0x96 => (8, Some(Instruction::SUB(ArithmeticTarget::HL))),
            0x97 => (4, Some(Instruction::SUB(ArithmeticTarget::A))),

            0x98 => (4, Some(Instruction::SBC(ArithmeticTarget::B))),
            0x99 => (4, Some(Instruction::SBC(ArithmeticTarget::C))),
            0x9A => (4, Some(Instruction::SBC(ArithmeticTarget::D))),
            0x9B => (4, Some(Instruction::SBC(ArithmeticTarget::E))),
            0x9C => (4, Some(Instruction::SBC(ArithmeticTarget::H))),
            0x9D => (4, Some(Instruction::SBC(ArithmeticTarget::L))),
            0x9E => (8, Some(Instruction::SBC(ArithmeticTarget::HL))),
            0x9F => (4, Some(Instruction::SBC(ArithmeticTarget::A))),

            0xA0 => (4, Some(Instruction::AND(ArithmeticTarget::B))),
            0xA1 => (4, Some(Instruction::AND(ArithmeticTarget::C))),
            0xA2 => (4, Some(Instruction::AND(ArithmeticTarget::D))),
            0xA3 => (4, Some(Instruction::AND(ArithmeticTarget::E))),
            0xA4 => (4, Some(Instruction::AND(ArithmeticTarget::H))),
            0xA5 => (4, Some(Instruction::AND(ArithmeticTarget::L))),
            0xA6 => (8, Some(Instruction::AND(ArithmeticTarget::HL))),
            0xA7 => (4, Some(Instruction::AND(ArithmeticTarget::A))),

            0xA8 => (4, Some(Instruction::XOR(ArithmeticTarget::B))),
            0xA9 => (4, Some(Instruction::XOR(ArithmeticTarget::C))),
            0xAA => (4, Some(Instruction::XOR(ArithmeticTarget::D))),
            0xAB => (4, Some(Instruction::XOR(ArithmeticTarget::E))),
            0xAC => (4, Some(Instruction::XOR(ArithmeticTarget::H))),
            0xAD => (4, Some(Instruction::XOR(ArithmeticTarget::L))),
            0xAE => (8, Some(Instruction::XOR(ArithmeticTarget::HL))),
            0xAF => (4, Some(Instruction::XOR(ArithmeticTarget::A))),

            0xB0 => (4, Some(Instruction::OR(ArithmeticTarget::B))),
            0xB1 => (4, Some(Instruction::OR(ArithmeticTarget::C))),
            0xB2 => (4, Some(Instruction::OR(ArithmeticTarget::D))),
            0xB3 => (4, Some(Instruction::OR(ArithmeticTarget::E))),
            0xB4 => (4, Some(Instruction::OR(ArithmeticTarget::H))),
            0xB5 => (4, Some(Instruction::OR(ArithmeticTarget::L))),
            0xB6 => (8, Some(Instruction::OR(ArithmeticTarget::HL))),
            0xB7 => (4, Some(Instruction::OR(ArithmeticTarget::A))),

            0xB8 => (4, Some(Instruction::CP(ArithmeticTarget::B))),
            0xB9 => (4, Some(Instruction::CP(ArithmeticTarget::C))),
            0xBA => (4, Some(Instruction::CP(ArithmeticTarget::D))),
            0xBB => (4, Some(Instruction::CP(ArithmeticTarget::E))),
            0xBC => (4, Some(Instruction::CP(ArithmeticTarget::H))),
            0xBD => (4, Some(Instruction::CP(ArithmeticTarget::L))),
            0xBE => (8, Some(Instruction::CP(ArithmeticTarget::HL))),
            0xBF => (4, Some(Instruction::CP(ArithmeticTarget::A))),

            0xC6 => (8, Some(Instruction::ADD(ArithmeticTarget::IMM8))),
            0xD6 => (8, Some(Instruction::SUB(ArithmeticTarget::IMM8))),
            0xE6 => (8, Some(Instruction::AND(ArithmeticTarget::IMM8))),
            0xF6 => (8, Some(Instruction::OR(ArithmeticTarget::IMM8))),

            0xCE => (8, Some(Instruction::ADC(ArithmeticTarget::IMM8))),
            0xDE => (8, Some(Instruction::SBC(ArithmeticTarget::IMM8))),
            0xEE => (8, Some(Instruction::XOR(ArithmeticTarget::IMM8))),
            0xFE => (8, Some(Instruction::CP(ArithmeticTarget::IMM8))),

            // 16 bit arithmetic

            0x03 => (8, Some(Instruction::INC16(ArithmeticTarget16::BC))),
            0x13 => (8, Some(Instruction::INC16(ArithmeticTarget16::DE))),
            0x23 => (8, Some(Instruction::INC16(ArithmeticTarget16::HL))),
            0x33 => (8, Some(Instruction::INC16(ArithmeticTarget16::SP))),

            0x09 => (8, Some(Instruction::ADD16(ArithmeticTarget16::BC))),
            0x19 => (8, Some(Instruction::ADD16(ArithmeticTarget16::DE))),
            0x29 => (8, Some(Instruction::ADD16(ArithmeticTarget16::HL))),
            0x39 => (8, Some(Instruction::ADD16(ArithmeticTarget16::SP))),

            0x0B => (8, Some(Instruction::DEC16(ArithmeticTarget16::BC))),
            0x1B => (8, Some(Instruction::DEC16(ArithmeticTarget16::DE))),
            0x2B => (8, Some(Instruction::DEC16(ArithmeticTarget16::HL))),
            0x3B => (8, Some(Instruction::DEC16(ArithmeticTarget16::SP))),

            0xE8 => (16, Some(Instruction::ADD16(ArithmeticTarget16::SPIMM))),

            // jumps (JP and JR are combined)

            0x20 => (8, Some(Instruction::JP(ControlCondition::NZ, JumpAddr::REL))),
            0x30 => (8, Some(Instruction::JP(ControlCondition::NC, JumpAddr::REL))),

            0x18 => (12, Some(Instruction::JP(ControlCondition::NONE, JumpAddr::REL))),
            0x28 => (8, Some(Instruction::JP(ControlCondition::Z, JumpAddr::REL))),
            0x38 => (8, Some(Instruction::JP(ControlCondition::C, JumpAddr::REL))),

            0xC2 => (12, Some(Instruction::JP(ControlCondition::NZ, JumpAddr::IMM16))),
            0xD2 => (12, Some(Instruction::JP(ControlCondition::NC, JumpAddr::IMM16))),
            0xC3 => (16, Some(Instruction::JP(ControlCondition::NONE, JumpAddr::IMM16))),

            0xE9 => (4, Some(Instruction::JP(ControlCondition::Z, JumpAddr::IMM16))),
            0xCA => (12, Some(Instruction::JP(ControlCondition::C, JumpAddr::IMM16))),

            // calls

            0xC4 => (12, Some(Instruction::CALL(ControlCondition::NZ))),
            0xD4 => (12, Some(Instruction::CALL(ControlCondition::NC))),
            0xCC => (12, Some(Instruction::CALL(ControlCondition::Z))),
            0xDC => (12, Some(Instruction::CALL(ControlCondition::C))),
            0xCD => (24, Some(Instruction::CALL(ControlCondition::NONE))),

            // returns

            0xC0 => (8, Some(Instruction::RET(ControlCondition::NZ))),
            0xD0 => (8, Some(Instruction::RET(ControlCondition::NC))),
            0xC8 => (8, Some(Instruction::RET(ControlCondition::Z))),
            0xD8 => (8, Some(Instruction::RET(ControlCondition::C))),
            0xC9 => (16, Some(Instruction::RET(ControlCondition::NONE))),
            0xD9 => (16, Some(Instruction::RET(ControlCondition::NONEEI))),

            // resets

            0xC7 => (16, Some(Instruction::RST(RstValue::H00))),
            0xD7 => (16, Some(Instruction::RST(RstValue::H10))),
            0xE7 => (16, Some(Instruction::RST(RstValue::H20))),
            0xF7 => (16, Some(Instruction::RST(RstValue::H30))),
            0xCF => (16, Some(Instruction::RST(RstValue::H08))),
            0xDF => (16, Some(Instruction::RST(RstValue::H18))),
            0xEF => (16, Some(Instruction::RST(RstValue::H28))),
            0xFF => (16, Some(Instruction::RST(RstValue::H38))),

            // misc

            0x00 => (4, Some(Instruction::NOP)),
            0x10 => (4, Some(Instruction::STOP)),
            0x76 => (4, Some(Instruction::HALT)),
            0xF3 => (4, Some(Instruction::DI)),
            0xFB => (4, Some(Instruction::EI)),

            0x27 => (4, Some(Instruction::DAA)),
            0x37 => (4, Some(Instruction::SCF)),
            0x2F => (4, Some(Instruction::CPL)),
            0x3F => (4, Some(Instruction::CCF)),

            _    => (0, None),
        }
    }

    fn from_byte_prefixed(byte: u8) -> (u16, Option<Instruction>) {
        match byte {
            _ => (0, None) // TODO: Implement prefixed opcodes
        }
    }
}
