use std::convert;

const ZERO_FLAG_BYTE_POSITION:       u8 = 7;
const SUBTRACT_FLAG_BYTE_POSITION:   u8 = 6;
const HALF_CARRY_FLAG_BYTE_POSITION: u8 = 5;
const CARRY_FLAG_BYTE_POSITION:      u8 = 4;

#[derive(Copy, Clone)]
pub struct FlagsRegister {
    pub zero:       bool,
    pub subtract:   bool,
    pub half_carry: bool,
    pub carry:      bool,
}

impl convert::From<FlagsRegister> for u8 {
    fn from(flags: FlagsRegister) -> u8 {
        (if flags.zero       { 1 } else { 0 }) << ZERO_FLAG_BYTE_POSITION |
        (if flags.subtract   { 1 } else { 0 }) << SUBTRACT_FLAG_BYTE_POSITION |
        (if flags.half_carry { 1 } else { 0 }) << HALF_CARRY_FLAG_BYTE_POSITION |
        (if flags.carry      { 1 } else { 0 }) << CARRY_FLAG_BYTE_POSITION
    }
}

impl convert::From<u8> for FlagsRegister {
    fn from(byte: u8) -> Self {
        let zero =       ((byte >> ZERO_FLAG_BYTE_POSITION) & 0b1) != 0;
        let subtract =   ((byte >> SUBTRACT_FLAG_BYTE_POSITION) & 0b1) != 0;
        let half_carry = ((byte >> HALF_CARRY_FLAG_BYTE_POSITION) & 0b1) != 0;
        let carry =      ((byte >> CARRY_FLAG_BYTE_POSITION) & 0b1) != 0;

        FlagsRegister {
            zero,
            subtract,
            half_carry,
            carry,
        }
    }
}

pub struct Registers {
    pub a: u8,
    pub b: u8,
    pub c: u8,
    pub d: u8,
    pub e: u8,
    pub f: FlagsRegister,
    pub h: u8,
    pub l: u8,
}

impl Registers {
    pub fn get_af(&self) -> u16 {
        (self.a as u16) << 8 | u8::from(self.f) as u16
    }
    pub fn set_af(&mut self, af: u16) {
        self.a = (af >> 8) as u8;
        self.f = FlagsRegister::from((af & 0xFF) as u8);
    }

    pub fn get_bc(&self) -> u16 {
        (self.b as u16) << 8 | self.c as u16
    }
    pub fn set_bc(&mut self, bc: u16) {
        self.b = (bc >> 8) as u8;
        self.c = (bc & 0xFF ) as u8;
    }

    pub fn get_de(&self) -> u16 {
        (self.d as u16) << 8 | self.e as u16
    }
    pub fn set_de(&mut self, de: u16) {
        self.d = (de >> 8) as u8;
        self.e = (de & 0xFF ) as u8;
    }

    pub fn get_hl(&self) -> u16 {
        (self.h as u16) << 8 | self.l as u16
    }
    pub fn set_hl(&mut self, hl: u16) {
        self.h = (hl >> 8) as u8;
        self.l = (hl & 0xFF ) as u8;
    }
}
