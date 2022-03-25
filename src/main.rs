mod cpu;
use cpu::CPU;
use cpu::MemoryBus;
use cpu::Registers;
use cpu::registers::FlagsRegister;

#[allow(dead_code)]
// Controller to run gameboy emulator
fn main() {
    let mut memory = MemoryBus {
        memory: [0; 0xFFFF],
    };

    memory.memory[0] = 0x00;

    // read instructions into memory

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

    let mut my_cpu = CPU {
        step_delay: 16750,
        frequency: 4194304,
        reg: my_registers,
        bus: memory,
        pc: 0,
        sp: 0xFFFF,
        interrupt_enable: false,
        is_halted: false,
        is_stopped: false,
    };

    my_cpu.step();
    assert!(my_cpu.is_halted);

    loop {
        my_cpu.step();
    }
}
