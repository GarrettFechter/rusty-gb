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
                    LoadType::Word(dest, source) => {
                        let source_value = match source {
                            LoadWordSource::BC  => self.reg.get_bc(),
                            LoadWordSource::DE  => self.reg.get_de(),
                            LoadWordSource::HL  => self.reg.get_hl(),
                            LoadWordSource::SP  => self.sp,
                            LoadWordSource::POP => {
                                let value = ((self.bus.read_byte(self.sp + 1)) as u16 << 8) | self.bus.read_byte(self.sp) as u16;
                                self.sp += 2;
                                value
                            }
                            LoadWordSource::AF => self.reg.get_af(),
                            LoadWordSource::IMM16 => {
                                self.bus.read_byte(self.pc + 1) as u16 | (self.bus.read_byte(self.pc + 2) as u16 << 8)
                            }
                            // there was a little ambiguity on the LDHL SP, e aka LD HL,SP+e
                            // instruction, but decided this was correct
                            LoadWordSource::SP_IMM => self.sp.wrapping_add(self.bus.read_byte(self.pc + 1) as u8)
                            _ => panic!("Unsupported LoadWordSource"),
                        }

                        match dest {
                            LoadWordDestination::BC => self.reg.set_bc(source_value),
                            LoadWordDestination::DE => self.reg.set_de(source_value),
                            LoadWordDestination::HL => self.reg.set_hl(source_value),
                            LoadWordDestination::SP => self.reg.sp = source_value,
                            LoadWordDestination::PUSH => {
                                self.bus.write_byte(self.reg.sp, source_value);
                                self.sp += 1;
                            },
                            LoadWordDestination::AF => self.reg.set_af(source_value),
                            LoadWordDestination::A16 => {
                                let addr = self.bus.read_byte(pc + 1) as u16 | (self.bus.read_byte(pc + 2) as u16 << 8);
                                self.bus.write_byte(addr, source_value);
                            }
                        }

                        // return next pc based on instruction length
                        match (dest, source) {
                            // should be no overlap
                            (_, LoadWordSource::IMM16) |
                            (LoadWordDestination::A16, _) => self.pc.wrapping_add(3),

                            (_, LoadWordSource::SP_IMM) => self.pc.wrapping_add(2),

                            (_, _) => self.pc.wrapping_add(1),
                        }
                    }
                    LoadType::Byte(dest, source) => {
                        let source_value = match source {
                            LoadByteSource::A    => self.reg.a,
                            LoadByteSource::B    => self.reg.b,
                            LoadByteSource::C    => self.reg.c,
                            LoadByteSource::D    => self.reg.d,
                            LoadByteSource::E    => self.reg.e,
                            LoadByteSource::H    => self.reg.h,
                            LoadByteSource::L    => self.reg.l,
                            LoadByteSource::HLI  => {
                                let s = self.bus.read_byte(self.reg.h << 8 || self.reg.l);
                                self.reg.set_hl(self.reg.get_hl().wrapping_add(1));
                                s
                            },
                            LoadByteSource::HLD  => {
                                let s = self.bus.read_byte(self.reg.h << 8 || self.reg.l);
                                self.reg.set_hl(self.reg.get_hl().wrapping_sub(1));
                                s
                            },
                            LoadByteSource::BC   => self.bus.read_byte(self.reg.get_bc()),
                            LoadByteSource::DE   => self.bus.read_byte(self.reg.get_de()),
                            LoadByteSource::HL   => self.bus.read_byte(self.reg.get_hl()),
                            LoadByteSource::A8   => self.bus.read_byte(0xFF00 | self.bus.read_byte(self.pc + 1)),
                            LoadByteSource::C_A8   => self.bus.read_byte(0xFF00 | self.reg.c),
                            LoadByteSource::A16  => {
                                // pc+1 holds LSB, pc+2 holds MSB
                                let addr = self.bus.read_byte(self.pc + 1) | (self.bus.read_byte(self.pc + 2) << 4);
                                self.bus.read_byte(addr)
                            },
                            LoadByteSource::IMM8 => self.bus.read_byte(self.pc + 1),
                        }

                        match dest {
                            LoadByteDestination::A   => self.reg.a = source_value,
                            LoadByteDestination::B   => self.reg.b = source_value,
                            LoadByteDestination::C   => self.reg.c = source_value,
                            LoadByteDestination::D   => self.reg.d = source_value,
                            LoadByteDestination::E   => self.reg.e = source_value,
                            LoadByteDestination::H   => self.reg.h = source_value,
                            LoadByteDestination::L   => self.reg.l = source_value,
                            LoadByteDestination::HLI => {
                                self.bus.write_byte(self.reg.get_hl, source_value);
                                self.reg.set_hl(self.reg.get_hl().wrapping_add(1));
                            },
                            LoadByteDestination::HLD => {
                                self.bus.write_byte(self.reg.get_hl, source_value);
                                self.reg.set_hl(self.reg.get_hl().wrapping_sub(1));
                            },
                            LoadByteDestination::BC  => self.bus.write_byte(self.reg.get_bc(), source_value),
                            LoadByteDestination::DE  => self.bus.write_byte(self.reg.get_de(), source_value),
                            LoadByteDestination::HL  => self.bus.write_byte(self.reg.get_hl(), source_value),
                            LoadByteDestination::A8  => {
                                let addr = 0xFF00 | self.bus.read_byte(self.pc + 1);
                                self.bus.write_byte(addr, source_value),
                            },
                            LoadByteDestination::C_A8  => {
                                let addr = 0xFF00 | self.reg.c;
                                self.bus.write_byte(addr, source_value),
                            },
                            LoadByteDestination::A16 => {
                                // pc+1 holds LSB, pc+2 holds MSB
                                let addr = self.bus.read_byte(self.pc + 1) | (self.bus.read_byte(self.pc + 2) << 4);
                                self.bus.write_byte(addr, source_value)
                            },
                        }

                        match (dest, source) {
                            // should be no overlap
                            (_, LoadByteSource::A8)   |
                            (_, LoadByteSource::IMM8) |
                            (LoadByteDestination::A8, _) => self.pc.wrapping_add(2),

                            (_, LoadByteSource::A16) |
                            (LoadByteDestination::A16, _) => self.pc.wrapping_add(3),

                            (_, _) => self.pc.wrapping_add(1),
                        }
                    }
                }
            }
            _ => { self.pc }, // TODO: implement other instructions
        }
    }

    // Reads and executes instruction at pc
    fn step(&mut self) {
        let mut instruction_byte = self.bus.read_byte(pc);
        let prefixed = instruction_byte == 0xCB;

        if prefixed {
            instruction_byte = self.bus.read_byte(pc + 1);
        }

        let next_pc = if let Some(instruction) = Instruction::from_byte(instruction_byte, prefixed) {
            self.execute(instruction)
        } else {
            let description = format!("0x{}{:x}", if prefixed { "CB" } else { "" }, instruction_byte);
            panic!("Unknown instruction {}", description);
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
