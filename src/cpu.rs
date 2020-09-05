struct MemoryBus {
    memory: [u8; 0xFFFF],
}

impl MemoryBus {
    fn read_byte(&self, address: u16) -> u8 {
        self.memory[address as usize]
    }
    fn write_byte(&mut self, address: u16, value: u8) {
        self.memory[address as usize] = value;
    }
}

struct CPU {
    reg: Registers,
    bus: MemoryBus,
    pc: u16,
    sp: u16,
}

impl CPU {
    // Executes given instruction and returns next pc
    fn execute(&mut self, instruction: Instruction) {
        match instruction {
            Instruction::ADD(target) => {
                match target {
                    ArithmeticTarget::C => {
                        let value = self.reg.c;
                        let new_value = self.add(value);
                        self.reg.a = new_value;
                        self.pc.wrapping_add(1)
                    }
                    _ => { self.pc } // TODO: implement ADD on other registers
                }
            }
            Instruction::LD(load_type) => {
                match load_type {
                    LoadType::Byte(dest, source) => {
                        let source_value = match source {
                            LoadSource::A    => self.reg.a,
                            LoadSource::B    => self.reg.b,
                            LoadSource::C    => self.reg.c,
                            LoadSource::D    => self.reg.d,
                            LoadSource::E    => self.reg.e,
                            LoadSource::H    => self.reg.h,
                            LoadSource::L    => self.reg.l,
                            LoadSource::HLI  => {
                                let s = self.bus.read_byte(self.reg.h << 8 || self.reg.l);
                                self.reg.set_hl(self.reg.get_hl().wrapping_add(1));
                                s
                            },
                            LoadSource::HLD  => {
                                let s = self.bus.read_byte(self.reg.h << 8 || self.reg.l);
                                self.reg.set_hl(self.reg.get_hl().wrapping_sub(1));
                                s
                            },
                            LoadSource::BC   => self.bus.read_byte(self.reg.get_bc()),
                            LoadSource::DE   => self.bus.read_byte(self.reg.get_de()),
                            LoadSource::HL   => self.bus.read_byte(self.reg.get_hl()),
                            LoadSource::A8   => self.bus.read_byte(0xFF00 | self.bus.read_byte(self.pc + 1)),
                            LoadSource::C_A8   => self.bus.read_byte(0xFF00 | self.reg.c),
                            LoadSource::A16  => {
                                // pc+1 holds LSB, pc+2 holds MSB
                                let addr = self.bus.read_byte(self.pc + 1) | (self.bus.read_byte(self.pc + 2) << 4);
                                self.bus.read_byte(addr)
                            },
                            LoadSource::IMM8 => self.bus.read_byte(self.pc + 1),
                        }

                        match dest {
                            LoadDestination::A   => self.reg.a = source_value,
                            LoadDestination::B   => self.reg.b = source_value,
                            LoadDestination::C   => self.reg.c = source_value,
                            LoadDestination::D   => self.reg.d = source_value,
                            LoadDestination::E   => self.reg.e = source_value,
                            LoadDestination::H   => self.reg.h = source_value,
                            LoadDestination::L   => self.reg.l = source_value,
                            LoadDestination::HLI => {
                                self.bus.write_byte(self.reg.get_hl, source_value);
                                self.reg.set_hl(self.reg.get_hl().wrapping_add(1));
                            },
                            LoadDestination::HLD => {
                                self.bus.write_byte(self.reg.get_hl, source_value);
                                self.reg.set_hl(self.reg.get_hl().wrapping_sub(1));
                            },
                            LoadDestination::BC  => self.bus.write_byte(self.reg.get_bc(), source_value),
                            LoadDestination::DE  => self.bus.write_byte(self.reg.get_de(), source_value),
                            LoadDestination::HL  => self.bus.write_byte(self.reg.get_hl(), source_value),
                            LoadDestination::A8  => {
                                let addr = 0xFF00 | self.bus.read_byte(self.pc + 1);
                                self.bus.write_byte(addr, source_value),
                            }
                            LoadDestination::C_A8  => {
                                let addr = 0xFF00 | self.reg.c;
                                self.bus.write_byte(addr, source_value),
                            }
                            LoadDestination::A16 => {
                                // pc+1 holds LSB, pc+2 holds MSB
                                let addr = self.bus.read_byte(self.pc + 1) | (self.bus.read_byte(self.pc + 2) << 4);
                                self.bus.write_byte(addr, source_value)
                            }
                        }

                        match (dest, source) {
                            // should be no overlap

                            (_, LoadSource::A8)   |
                            (_, LoadSource::IMM8) |
                            (LoadDestination::A8, _) => self.pc.wrapping_add(2),

                            (_, LoadSource::A16) |
                            (LoadDestination::A16, _) => self.pc.wrapping_add(3),

                            (_, _) => self.pc.wrapping_add(1),
                        }
                    }
                }
            }
            }
            _ => { self.pc } // TODO: implement other instructions
        }
    }

    // Reads and executes instruction at pc
    fn step(&mut self) {
        let mut instruction_byte = self.bus.read_byte(pc);

        let next_pc = if let Some(instruction) = Instruction::from_byte(instruction_byte) {
            self.execute(instruction)
        } else {
            panic!("Unknown instruction 0x{:x}", instruction_byte);
        };

        self.pc = next_pc;
    }

    fn add(&mut self, value: u8) -> u8 {
        let (new_value, did_overflow) = self.reg.a.overflowing_add(value);
        self.reg.f.zero = new_value == 0;
        self.reg.f.subtract = false;
        self.reg.f.carry = did_overflow;
        self.reg.f.half_carry = (self.reg.a & 0xF) + (value & 0xF) > 0xF;
        new_value
    }
}
