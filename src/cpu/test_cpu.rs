use super::*;

#[test]
fn test_add() {
    let mut cpu = CPU::new();
    assert_eq!(cpu.add(0xFF), 0xFF);
    cpu.reg.a = 0xF0;
    assert_eq!(cpu.add(0x0F), 0xFF);
}
