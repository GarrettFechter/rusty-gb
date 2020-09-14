mod cpu;
use cpu::CPU;
use cpu::MemoryBus;
use cpu::Registers;
use cpu::registers::FlagsRegister;

#[allow(dead_code)]
fn main() {
    let my_memorybus = MemoryBus {
        memory: [0; 0xFFFF],
    };

    let fr = FlagsRegister {
        zero:       false,
        subtract:   false,
        half_carry: false,
        carry:      false,
    };

    let my_registers = Registers {
        a: 0,
        b: 0,
        c: 0,
        d: 0,
        e: 0,
        f: fr,
        h: 0,
        l: 0,
    };

    let my_cpu = CPU {
        reg: my_registers,
        bus: my_memorybus,
        pc: 0,
        sp: 0xFFFF,
        interrupt_enable: false,
        is_halted: false,
        is_stopped: false,
    };

    assert!(my_cpu.is_halted);
}
