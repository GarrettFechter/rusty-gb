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
    cpu.reg.f.carry = true;
    cpu.reg.a = 0x01;
    let adc = cpu.adc(0xFD);
    assert_eq!(adc, 0xFF);
    cpu.reg.f.carry = false;
    cpu.check_zero_reg(adc);
    cpu.check_subtract_false();
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

#[test]
fn test_sbc() {
    let mut cpu = CPU::new();
    cpu.reg.f.carry = true;
    cpu.reg.a = 0x02;
    let sbc = cpu.sbc(0x01);
    assert_eq!(sbc, 0x00);
    cpu.reg.f.carry = false;
    cpu.check_subtract_true();
    cpu.check_zero_reg(sbc);
    cpu.reg.f.carry = false;
    cpu.reg.a = 0x01;
    let sbc = cpu.sbc(0x02);
    assert_eq!(sbc, 0xFF);
    cpu.check_zero_reg(sbc);
    cpu.check_subtract_true();
    assert_eq!(cpu.reg.f.carry, true)
}

#[test]
fn test_and() {
    let mut cpu = CPU::new();
    cpu.reg.a = 0xF0;
    assert_eq!(0x00, cpu.and(0x0F));
}

#[test]
fn test_or() {
    let mut cpu = CPU::new();
    cpu.reg.a = 0xF0;
    assert_eq!(0xFF, cpu.or(0x0F));
}

#[test]
fn test_xor() {
    let mut cpu = CPU::new();
    cpu.reg.a = 0xFF;
    assert_eq!(0x55, cpu.xor(0xAA));
}

#[test]
fn test_inc() {
    let mut cpu = CPU::new();
    assert_eq!(0x01, cpu.inc(0x00));
    assert_eq!(0x02, cpu.inc(0x01));
    assert_eq!(cpu.reg.f.carry, false);
    assert_eq!(0x00, cpu.inc(0xFF));
    cpu.check_subtract_false();
}

#[test]
fn test_dec() {
    let mut cpu = CPU::new();
    assert_eq!(0x01, cpu.dec(0x02));
    assert_eq!(0x00, cpu.dec(0x01));
    assert_eq!(cpu.reg.f.carry, false);
    assert_eq!(0xFF, cpu.dec(0x00));
    cpu.check_subtract_true();
}

#[test]
fn test_push() {
    let mut cpu = CPU::new();
    cpu.push(0x11);
    assert_eq!(cpu.bus.read_byte(cpu.sp), 0x11);
}

#[test]
fn test_push_pop() {
    let mut cpu = CPU::new();
    for i in 0..10 {
        cpu.push(i);
    }
    assert_eq!(cpu.pop(), 0x09);
    cpu.push(0x22);
    assert_eq!(cpu.pop(), 0x22);
    assert_eq!(0x08, cpu.pop());
    assert_eq!(0x07, cpu.pop());
    assert_eq!(0x06, cpu.pop());
    assert_eq!(0x05, cpu.pop());
    assert_eq!(0x04, cpu.pop());
    cpu.push(0x22);
    assert_eq!(cpu.pop(), 0x22);
    assert_eq!(0x03, cpu.pop());
    assert_eq!(0x02, cpu.pop());
    assert_eq!(0x01, cpu.pop());
    assert_eq!(0x00, cpu.pop());
}

// test Instructions with cpu.execute()

#[test]
fn test_ld() {
    let mut cpu = CPU::new();
    let instruction = Instruction::LD(LoadType::Byte(LoadByteDestination::A, LoadByteSource::B));
    cpu.reg.b = 0xF1;
    cpu.is_halted = false;
    cpu.is_stopped = false;
    cpu.execute(instruction);
    assert_eq!(cpu.reg.b, cpu.reg.a);
    assert_eq!(cpu.reg.a, 0xF1);
}
