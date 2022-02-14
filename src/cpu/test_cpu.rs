use super::*;

impl CPU {
    fn check_zero_reg(&self, value: u8) { assert_eq!(self.reg.f.zero, value == 0); }
    fn check_subtract_true(&self)       { assert_eq!(self.reg.f.subtract, true); }
    fn check_subtract_false(&self)      { assert_eq!(self.reg.f.subtract, false); }
}

// test CPU internal helpers directly

#[test]
fn test_add() {
    let mut cpu = CPU::new();
    // test add empty
    let add = cpu.add(0xFF);
    assert_eq!(add, 0xFF);
    assert_eq!(cpu.reg.f.carry, false);
    cpu.check_zero_reg(add);
    cpu.check_subtract_false();

    // test add non empty
    cpu.reg.a = 0xF0;
    let add = cpu.add(0x0F);
    assert_eq!(add, 0xFF);
    assert_eq!(cpu.reg.f.carry, false);
    cpu.check_zero_reg(add);

    // test add overflowing
    cpu.reg.a = 0xFF;
    let add = cpu.add(0x1);
    assert_eq!(add, 0x00);
    assert_eq!(cpu.reg.f.carry, true);
    cpu.reg.a = 0xFF;
    assert_eq!(cpu.add(0x1), 0x00);
    assert_eq!(cpu.reg.f.carry, true);
    cpu.check_subtract_false();
}

#[test]
fn test_add16() {
    let mut cpu = CPU::new();
    let add = cpu.add16(0x11, 0x22);
    assert_eq!(add, 0x33);
    cpu.check_subtract_false();
}

#[test]
fn test_adc() {
    let mut cpu = CPU::new();
    cpu.reg.a = 0x01;
    let adc = cpu.adc(0xFE);
    assert_eq!(adc, 0xFF);
    cpu.check_zero_reg(adc);
    cpu.reg.a = 0x11;
    let adc = cpu.adc(0xEF);
    assert_eq!(adc, 0x00);
    cpu.check_zero_reg(adc);
}

#[test]
fn test_sub() {
    let mut cpu = CPU::new();
    cpu.reg.a = 0x01;
    let sub = cpu.sub(0x01);
    assert_eq!(sub, 0x00);
    cpu.check_subtract_true();
    cpu.check_zero_reg(sub);
    cpu.reg.a = 0x11;
    let sub = cpu.sub(0x12);
    assert_eq!(sub, 0xFF);
    assert_eq!(cpu.reg.f.carry, true);
}

// test Instructions
