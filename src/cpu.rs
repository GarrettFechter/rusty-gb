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
                    ArithmeticTarget::B    => self.reg.a = self.add(self.reg.b),
                    ArithmeticTarget::C    => self.reg.a = self.add(self.reg.c),
                    ArithmeticTarget::D    => self.reg.a = self.add(self.reg.d),
                    ArithmeticTarget::E    => self.reg.a = self.add(self.reg.e),
                    ArithmeticTarget::H    => self.reg.a = self.add(self.reg.h),
                    ArithmeticTarget::L    => self.reg.a = self.add(self.reg.l),
                    ArithmeticTarget::HL   => self.reg.a = self.add(self.bus.read_byte(self.reg.get_hl())),
                    ArithmeticTarget::A    => self.reg.a = self.add(self.reg.a),
                    ArithmeticTarget::IMM8 => self.reg.a = self.add(self.bus.read_byte(self.pc + 1)),
                    _ => panic!("Undefined SUB ArithmeticTarget"),
                };
                // return next PC value
                match target {
                    ArithmeticTarget::IMM8 => self.pc + 2,
                    _ => self.pc + 1,
                }
            }
            Instruction::SUB(target) => {
                match target {
                    ArithmeticTarget::B    => self.reg.a = self.sub(self.reg.b),
                    ArithmeticTarget::C    => self.reg.a = self.sub(self.reg.c),
                    ArithmeticTarget::D    => self.reg.a = self.sub(self.reg.d),
                    ArithmeticTarget::E    => self.reg.a = self.sub(self.reg.e),
                    ArithmeticTarget::H    => self.reg.a = self.sub(self.reg.h),
                    ArithmeticTarget::L    => self.reg.a = self.sub(self.reg.l),
                    ArithmeticTarget::HL   => self.reg.a = self.sub(self.bus.read_byte(self.reg.get_hl())),
                    ArithmeticTarget::A    => self.reg.a = self.sub(self.reg.a),
                    ArithmeticTarget::IMM8 => self.reg.a = self.sub(self.bus.read_byte(self.pc + 1)),
                    _ => panic!("Undefined SUB ArithmeticTarget"),
                };
                // return next PC value
                match target {
                    ArithmeticTarget::IMM8 => self.pc + 2,
                    _ => self.pc + 1,
                }
            }
            Instruction::AND(target) => {
                match target {
                    ArithmeticTarget::B    => self.reg.a = self.and(self.reg.b),
                    ArithmeticTarget::C    => self.reg.a = self.and(self.reg.c),
                    ArithmeticTarget::D    => self.reg.a = self.and(self.reg.d),
                    ArithmeticTarget::E    => self.reg.a = self.and(self.reg.e),
                    ArithmeticTarget::H    => self.reg.a = self.and(self.reg.h),
                    ArithmeticTarget::L    => self.reg.a = self.and(self.reg.l),
                    ArithmeticTarget::HL   => self.reg.a = self.and(self.bus.read_byte(self.reg.get_hl())),
                    ArithmeticTarget::A    => self.reg.a = self.and(self.reg.a),
                    ArithmeticTarget::IMM8 => self.reg.a = self.and(self.bus.read_byte(self.pc + 1)),
                    _ => panic!("Undefined AND ArithmeticTarget"),
                };
                // return next PC value
                match target {
                    ArithmeticTarget::IMM8 => self.pc + 2,
                    _ => self.pc + 1,
                }
            }
            Instruction::OR(target) => {
                match target {
                    ArithmeticTarget::B    => self.reg.a = self.or(self.reg.b),
                    ArithmeticTarget::C    => self.reg.a = self.or(self.reg.c),
                    ArithmeticTarget::D    => self.reg.a = self.or(self.reg.d),
                    ArithmeticTarget::E    => self.reg.a = self.or(self.reg.e),
                    ArithmeticTarget::H    => self.reg.a = self.or(self.reg.h),
                    ArithmeticTarget::L    => self.reg.a = self.or(self.reg.l),
                    ArithmeticTarget::HL   => self.reg.a = self.or(self.bus.read_byte(self.reg.get_hl())),
                    ArithmeticTarget::A    => self.reg.a = self.or(self.reg.a),
                    ArithmeticTarget::IMM8 => self.reg.a = self.or(self.bus.read_byte(self.pc + 1)),
                    _ => panic!("Undefined OR ArithmeticTarget"),
                };
                // return next PC value
                match target {
                    ArithmeticTarget::IMM8 => self.pc + 2,
                    _ => self.pc + 1,
                }
            }
            Instruction::ADC(target) => {
                match target {
                    ArithmeticTarget::B    => self.reg.a = self.adc(self.reg.b),
                    ArithmeticTarget::C    => self.reg.a = self.adc(self.reg.c),
                    ArithmeticTarget::D    => self.reg.a = self.adc(self.reg.d),
                    ArithmeticTarget::E    => self.reg.a = self.adc(self.reg.e),
                    ArithmeticTarget::H    => self.reg.a = self.adc(self.reg.h),
                    ArithmeticTarget::L    => self.reg.a = self.adc(self.reg.l),
                    ArithmeticTarget::HL   => self.reg.a = self.adc(self.bus.read_byte(self.reg.get_hl())),
                    ArithmeticTarget::A    => self.reg.a = self.adc(self.reg.a),
                    ArithmeticTarget::IMM8 => self.reg.a = self.adc(self.bus.read_byte(self.pc + 1)),
                    _ => panic!("Undefined ADC ArithmeticTarget"),
                };
                // return next PC value
                match target {
                    ArithmeticTarget::IMM8 => self.pc + 2,
                    _ => self.pc + 1,
                }
            }
            Instruction::SBC(target) => {
                match target {
                    ArithmeticTarget::B    => self.reg.a = self.sbc(self.reg.b),
                    ArithmeticTarget::C    => self.reg.a = self.sbc(self.reg.c),
                    ArithmeticTarget::D    => self.reg.a = self.sbc(self.reg.d),
                    ArithmeticTarget::E    => self.reg.a = self.sbc(self.reg.e),
                    ArithmeticTarget::H    => self.reg.a = self.sbc(self.reg.h),
                    ArithmeticTarget::L    => self.reg.a = self.sbc(self.reg.l),
                    ArithmeticTarget::HL   => self.reg.a = self.sbc(self.bus.read_byte(self.reg.get_hl())),
                    ArithmeticTarget::A    => self.reg.a = self.sbc(self.reg.a),
                    ArithmeticTarget::IMM8 => self.reg.a = self.sbc(self.bus.read_byte(self.pc + 1)),
                    _ => panic!("Undefined SBC ArithmeticTarget"),
                };
                // return next PC value
                match target {
                    ArithmeticTarget::IMM8 => self.pc + 2,
                    _ => self.pc + 1,
                }
            }
            Instruction::XOR(target) => {
                match target {
                    ArithmeticTarget::B    => self.reg.a = self.xor(self.reg.b),
                    ArithmeticTarget::C    => self.reg.a = self.xor(self.reg.c),
                    ArithmeticTarget::D    => self.reg.a = self.xor(self.reg.d),
                    ArithmeticTarget::E    => self.reg.a = self.xor(self.reg.e),
                    ArithmeticTarget::H    => self.reg.a = self.xor(self.reg.h),
                    ArithmeticTarget::L    => self.reg.a = self.xor(self.reg.l),
                    ArithmeticTarget::HL   => self.reg.a = self.xor(self.bus.read_byte(self.reg.get_hl())),
                    ArithmeticTarget::A    => self.reg.a = self.xor(self.reg.a),
                    ArithmeticTarget::IMM8 => self.reg.a = self.xor(self.bus.read_byte(self.pc + 1)),
                    _ => panic!("Undefined XOR ArithmeticTarget"),
                };
                // return next PC value
                match target {
                    ArithmeticTarget::IMM8 => self.pc + 2,
                    _ => self.pc + 1,
                }
            }
            Instruction::CP(target) => {
                match target {
                    // perform subtract but only update flags
                    ArithmeticTarget::B    => self.sub(self.reg.b),
                    ArithmeticTarget::C    => self.sub(self.reg.c),
                    ArithmeticTarget::D    => self.sub(self.reg.d),
                    ArithmeticTarget::E    => self.sub(self.reg.e),
                    ArithmeticTarget::H    => self.sub(self.reg.h),
                    ArithmeticTarget::L    => self.sub(self.reg.l),
                    ArithmeticTarget::HL   => self.sub(self.bus.read_byte(self.reg.get_hl())),
                    ArithmeticTarget::A    => self.sub(self.reg.a),
                    ArithmeticTarget::IMM8 => self.sub(self.bus.read_byte(self.pc + 1)),
                    _ => panic!("Undefined CP ArithmeticTarget"),
                };
                // return next PC value
                match target {
                    ArithmeticTarget::IMM8 => self.pc + 2,
                    _ => self.pc + 1,
                }
            }
            Instruction::INC(target) => {
                match target {
                    ArithmeticTarget::B    => self.reg.b = self.inc(self.reg.b),
                    ArithmeticTarget::C    => self.reg.c = self.inc(self.reg.c),
                    ArithmeticTarget::D    => self.reg.d = self.inc(self.reg.d),
                    ArithmeticTarget::E    => self.reg.e = self.inc(self.reg.e),
                    ArithmeticTarget::H    => self.reg.h = self.inc(self.reg.h),
                    ArithmeticTarget::L    => self.reg.l = self.inc(self.reg.l),
                    ArithmeticTarget::HL   => {
                        let addr = self.reg.get_hl();
                        self.bus.write_byte(addr, self.inc(self.bus.read_byte(addr));
                    },
                    ArithmeticTarget::A    => self.reg.a = self.inc(self.reg.a),
                    _ => panic!("Undefined INC ArithmeticTarget"),
                };
                _ => self.pc + 1
            }
            Instruction::DEC(target) => {
                match target {
                    ArithmeticTarget::B    => self.reg.b = self.dec(self.reg.b),
                    ArithmeticTarget::C    => self.reg.c = self.dec(self.reg.c),
                    ArithmeticTarget::D    => self.reg.d = self.dec(self.reg.d),
                    ArithmeticTarget::E    => self.reg.e = self.dec(self.reg.e),
                    ArithmeticTarget::H    => self.reg.h = self.dec(self.reg.h),
                    ArithmeticTarget::L    => self.reg.l = self.dec(self.reg.l),
                    ArithmeticTarget::HL   => {
                        let addr = self.reg.get_hl();
                        self.bus.write_byte(addr, self.dec(self.bus.read_byte(addr));
                    },
                    ArithmeticTarget::A    => self.reg.a = self.dec(self.reg.a),
                    _ => panic!("Undefined DEC ArithmeticTarget"),
                };
                _ => self.pc + 1
            }

            Instruction::INC16(target) => {
                match target {
                    ArithmeticTarget::BC => self.reg.set_bc(self.reg.get_bc().wrapping_add(1)),
                    ArithmeticTarget::DE => self.reg.set_de(self.reg.get_de().wrapping_add(1)),
                    ArithmeticTarget::HL => self.reg.set_hl(self.reg.get_hl().wrapping_add(1)),
                    ArithmeticTarget::SP => self.sp = self.sp.wrapping_add(1),
                    _ => panic!("Undefined INC16 ArithmeticTarget"),
                };
                _ => self.pc + 1
            }

            Instruction::ADD16(target) => {
                match target {
                    ArithmeticTarget::BC      => self.reg.set_hl(self.add16(self.reg.get_hl(), self.reg.get_bc())),
                    ArithmeticTarget::DE      => self.reg.set_hl(self.add16(self.reg.get_hl(), self.reg.get_de())),
                    ArithmeticTarget::HL      => self.reg.set_hl(self.add16(self.reg.get_hl(), self.reg.get_hl())),
                    ArithmeticTarget::SP      => self.reg.set_hl(self.add16(self.reg.get_hl(), self.sp)),
                    ArithmeticTarget::SP_IMM  => {
                        let imm: i8 = self.bus.read_byte(pc + 1) as i8;
                        self.sp = self.sp.wrapping_add(imm as u16);
                    }
                    _ => panic!("Undefined ADD16 ArithmeticTarget"),
                };
                // return next PC value
                match target {
                    ArithmeticTarget::SP_IMM => self.pc + 2,
                    _ => self.pc + 1,
                }
            }

            Instruction::DEC16(target) => {
                match target {
                    ArithmeticTarget::BC => self.reg.set_bc(self.reg.get_bc().wrapping_sub(1)),
                    ArithmeticTarget::DE => self.reg.set_de(self.reg.get_de().wrapping_sub(1)),
                    ArithmeticTarget::HL => self.reg.set_hl(self.reg.get_hl().wrapping_sub(1)),
                    ArithmeticTarget::SP => self.sp = self.sp.wrapping_sub(1),
                    _ => panic!("Undefined DEC16 ArithmeticTarget"),
                };
                _ => self.pc + 1
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
                            },
                            LoadWordSource::AF => self.reg.get_af(),
                            LoadWordSource::IMM16 => {
                                self.bus.read_byte(self.pc + 1) as u16 | (self.bus.read_byte(self.pc + 2) as u16 << 8)
                            },
                            // there was a little ambiguity on the LDHL SP, e aka LD HL,SP+e
                            // instruction, but decided this was correct
                            LoadWordSource::SP_IMM => self.sp.wrapping_add(self.bus.read_byte(self.pc + 1) as u8),
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

    // Return A + value and update flags: Z 0 H C
    fn add(&mut self, value: u8) -> u8 {
        let (new_value, did_overflow) = self.reg.a.overflowing_add(value);
        self.reg.f.zero = new_value == 0;
        self.reg.f.subtract = false;
        self.reg.f.half_carry = (self.reg.a & 0xF) + (value & 0xF) > 0xF;
        self.reg.f.carry = did_overflow;
        new_value
    }

    // Return value1 + value2 and update flags: - 0 H C
    fn add16(&mut self, v1: u16, v2: u16) -> u16 {
        let (new_value, did_overflow) = v1.overflowing_add(v2);
        self.reg.f.subtract = false;
        // here, half carry is set if carry comes out of bit 11
        self.reg.f.half_carry = (v1 & 0xFFF) + (v2 & 0xFFF) > 0xFFF;
        self.reg.f.carry = did_overflow;
        new_value
    }

    // Return A + value + carry and update flags: Z 0 H C
    fn adc(&mut self, value: u8) -> u8 {
        let (first, did_overflow_first) = self.reg.a.overflowing_add(value);
        let (second, did_overflow_second) = first.overflowing_add(self.reg.f.carry as u8);
        self.reg.f.zero = second == 0;
        self.reg.f.subtract = false;
        self.reg.f.half_carry = (self.reg.a & 0xF) + (value & 0xF) + (self.reg.f.carry as u8) > 0xF;
        self.reg.f.carry = did_overflow_first || did_overflow_second;
        second
    }

    // Return A - value and update flags: Z 1 H C
    fn sub(&mut self, value: u8) -> u8 {
        let (new_value, did_overflow) = self.reg.a.overflowing_sub(value);
        self.reg.f.zero = new_value == 0;
        self.reg.f.subtract = true;
        self.reg.f.half_carry = (self.reg.a & 0xF) < (value & 0xF);
        self.reg.f.carry = did_overflow;
        new_value
    }

    // Return A - value - carry and update flags: Z 1 H C
    fn sbc(&mut self, value: u8) -> u8 {
        let (first, did_overflow_first) = self.reg.a.overflowing_sub(value);
        let (second, did_overflow_second) = first.overflowing_sub(self.reg.f.carry as u8);
        self.reg.f.zero = second == 0;
        self.reg.f.subtract = true;
        self.reg.f.half_carry = (self.reg.a & 0xF) < (value & 0xF) + (self.reg.f.carry as u8);
        self.reg.f.carry = did_overflow_first || did_overflow_second;
        second
    }

    // Return A & value and update flags: Z 0 1 0
    fn and(&mut self, value: u8) -> u8 {
        let new_value = self.reg.a & value;
        self.reg.f.zero = new_value == 0;
        self.reg.f.subtract = false;
        self.reg.f.half_carry = true;
        self.reg.f.carry = false;
        new_value
    }

    // Return A | value and update flags: Z 0 0 0
    fn or(&mut self, value: u8) -> u8 {
        let new_value = self.reg.a | value;
        self.reg.f.zero = new_value == 0;
        self.reg.f.subtract = false;
        self.reg.f.half_carry = false;
        self.reg.f.carry = false;
        new_value
    }

    // Return A ^ value and update flags: Z 0 0 0
    fn xor(&mut self, value: u8) -> u8 {
        let new_value = self.reg.a ^ value;
        self.reg.f.zero = new_value == 0;
        self.reg.f.subtract = false;
        self.reg.f.half_carry = false;
        self.reg.f.carry = false;
        new_value
    }

    // Return value + 1 and update flags: Z 0 H -
    fn inc(&mut self, value: u8) -> u8 {
        let (new_value, did_overflow) = value.overflowing_add(1);
        self.reg.f.zero = new_value == 0;
        self.reg.f.subtract = false;
        self.reg.f.half_carry = value == 0xF;
        new_value
    }

    // Return value - 1 and update flags: Z 1 H -
    fn dec(&mut self, value: u8) -> u8 {
        let (new_value, did_overflow) = value.overflowing_sub(1);
        self.reg.f.zero = new_value == 0;
        self.reg.f.subtract = true;
        self.reg.f.half_carry = value & 0xF == 0;
        new_value
    }
}
