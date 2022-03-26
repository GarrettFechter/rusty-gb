pub mod instructions;
pub use self::instructions::Instruction;
pub use self::instructions::LoadByteDestination;
pub use self::instructions::LoadByteSource;
pub use self::instructions::LoadWordDestination;
pub use self::instructions::LoadWordSource;
pub use self::instructions::LoadType;
pub use self::instructions::ArithmeticTarget;
pub use self::instructions::ArithmeticTarget16;
pub use self::instructions::ControlCondition;
pub use self::instructions::RstValue;
pub use self::instructions::JumpAddr;

pub mod instruction_cycle_table;
pub use self::instruction_cycle_table::get_cycle_count;

pub mod registers;
pub use self::registers::Registers;
pub use self::registers::FlagsRegister;

pub struct MemoryBus {
    pub memory: [u8; 0xFFFF],
}

impl MemoryBus {
    fn read_byte(&self, address: u16) -> u8 {
        self.memory[address as usize]
    }
    fn write_byte(&mut self, address: u16, value: u8) {
        self.memory[address as usize] = value;
    }
}

pub struct CPU {
    pub frequency: u64, // Hz
    pub frame_delay: u64, // microseconds aka (1 / fps) * 1000 * 1000
    pub reg: Registers,
    pub bus: MemoryBus,
    pub pc: u16,
    pub sp: u16,
    pub interrupt_enable: bool,
    pub is_halted: bool,
    pub is_stopped: bool,
}

impl CPU {
    fn new() -> CPU {
        // create a CPU with default values
        CPU {
            frequency: 4194304,
            frame_delay: 16750,
            reg: Registers {
            a: 0,
            b: 0,
            c: 0,
            d: 0,
            e: 0,
            f: FlagsRegister{
                zero:       false,
                subtract:   false,
                half_carry: false,
                carry:      false,
            },
            h: 0,
            l: 0,
        },
        bus: MemoryBus {
            memory: [0; 0xFFFF],
        },
        pc: 0,
        sp: 0xFFFF,
        interrupt_enable: true,
        is_halted: true,
        is_stopped: true,
        }
    }

    // Executes given instruction and returns (next pc, extra_cycles)
    fn execute(&mut self, instruction: Instruction) -> (u16, u8) {
        if self.is_stopped || self.is_halted {
            return (self.pc, 0);
        }

        let mut extra_cycles = 0;

        let next_pc = match instruction {
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
                        let value = self.inc(self.bus.read_byte(addr));
                        self.bus.write_byte(addr, value);
                    },
                    ArithmeticTarget::A    => self.reg.a = self.inc(self.reg.a),
                    _ => panic!("Undefined INC ArithmeticTarget"),
                };
                self.pc + 1
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
                        let value = self.dec(self.bus.read_byte(addr));
                        self.bus.write_byte(addr, value);
                    },
                    ArithmeticTarget::A    => self.reg.a = self.dec(self.reg.a),
                    _ => panic!("Undefined DEC ArithmeticTarget"),
                };
                self.pc + 1
            }

            Instruction::INC16(target) => {
                match target {
                    ArithmeticTarget16::BC => self.reg.set_bc(self.reg.get_bc().wrapping_add(1)),
                    ArithmeticTarget16::DE => self.reg.set_de(self.reg.get_de().wrapping_add(1)),
                    ArithmeticTarget16::HL => self.reg.set_hl(self.reg.get_hl().wrapping_add(1)),
                    ArithmeticTarget16::SP => self.sp = self.sp.wrapping_add(1),
                    _ => panic!("Undefined INC16 ArithmeticTarget"),
                };
                self.pc + 1
            }

            Instruction::ADD16(target) => {
                match target {
                    ArithmeticTarget16::BC      => {
                        let value = self.add16(self.reg.get_hl(), self.reg.get_bc());
                        self.reg.set_hl(value);
                    }
                    ArithmeticTarget16::DE      => {
                        let value = self.add16(self.reg.get_hl(), self.reg.get_de());
                        self.reg.set_hl(value);
                    }
                    ArithmeticTarget16::HL      => {
                        let value = self.add16(self.reg.get_hl(), self.reg.get_hl());
                        self.reg.set_hl(value);
                    }
                    ArithmeticTarget16::SP      => {
                        let value = self.add16(self.reg.get_hl(), self.sp);
                        self.reg.set_hl(value);
                    }
                    ArithmeticTarget16::SPIMM  => {
                        let imm: i8 = self.bus.read_byte(self.pc + 1) as i8;
                        self.sp = self.sp.wrapping_add(imm as u16);
                    }
                };
                // return next PC value
                match target {
                    ArithmeticTarget16::SPIMM => self.pc + 2,
                    _ => self.pc + 1,
                }
            }

            Instruction::DEC16(target) => {
                match target {
                    ArithmeticTarget16::BC => self.reg.set_bc(self.reg.get_bc().wrapping_sub(1)),
                    ArithmeticTarget16::DE => self.reg.set_de(self.reg.get_de().wrapping_sub(1)),
                    ArithmeticTarget16::HL => self.reg.set_hl(self.reg.get_hl().wrapping_sub(1)),
                    ArithmeticTarget16::SP => self.sp = self.sp.wrapping_sub(1),
                    _ => panic!("Undefined DEC16 ArithmeticTarget"),
                };
                self.pc + 1
            }

            Instruction::LD(load_type) => {
                match load_type {
                    LoadType::Word(dest, source) => {
                        let source_value = match source {
                            LoadWordSource::BC  => self.reg.get_bc(),
                            LoadWordSource::DE  => self.reg.get_de(),
                            LoadWordSource::HL  => self.reg.get_hl(),
                            LoadWordSource::SP  => self.sp,
                            LoadWordSource::POP => ((self.pop() as u16) << 8) | (self.pop() as u16),
                            LoadWordSource::AF => self.reg.get_af(),
                            LoadWordSource::IMM16 => {
                                self.bus.read_byte(self.pc + 1) as u16 | ((self.bus.read_byte(self.pc + 2) as u16) << 8)
                            },
                            LoadWordSource::SPIMM => {
                                let value = self.add16(self.sp, self.bus.read_byte(self.pc + 1) as u16);
                                self.reg.f.zero = false;
                                value
                            }
                        };

                        match dest {
                            LoadWordDestination::BC => self.reg.set_bc(source_value),
                            LoadWordDestination::DE => self.reg.set_de(source_value),
                            LoadWordDestination::HL => self.reg.set_hl(source_value),
                            LoadWordDestination::SP => self.sp = source_value,
                            LoadWordDestination::PUSH => {
                                self.push((source_value >> 8) as u8);
                                self.push((source_value & 0xFF) as u8);
                            },
                            LoadWordDestination::AF => self.reg.set_af(source_value),
                            LoadWordDestination::A16 => {
                                let addr = self.bus.read_byte(self.pc + 1) as u16 | ((self.bus.read_byte(self.pc + 2) as u16) << 8);
                                self.bus.write_byte(addr, (source_value & 0xFF) as u8);
                                self.bus.write_byte(addr + 1, (source_value >> 8) as u8);
                            }
                        }

                        // return next pc based on instruction length
                        match (dest, source) {
                            // should be no overlap
                            (_, LoadWordSource::IMM16) |
                            (LoadWordDestination::A16, _) => self.pc.wrapping_add(3),

                            (_, LoadWordSource::SPIMM) => self.pc.wrapping_add(2),

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
                                let s = self.bus.read_byte((self.reg.h as u16) << 8 | self.reg.l as u16);
                                self.reg.set_hl(self.reg.get_hl().wrapping_add(1));
                                s
                            },
                            LoadByteSource::HLD  => {
                                let s = self.bus.read_byte((self.reg.h as u16) << 8 | self.reg.l as u16);
                                self.reg.set_hl(self.reg.get_hl().wrapping_sub(1));
                                s
                            },
                            LoadByteSource::BC   => self.bus.read_byte(self.reg.get_bc()),
                            LoadByteSource::DE   => self.bus.read_byte(self.reg.get_de()),
                            LoadByteSource::HL   => self.bus.read_byte(self.reg.get_hl()),
                            LoadByteSource::A8   => self.bus.read_byte(0xFF00 | self.bus.read_byte(self.pc + 1) as u16),
                            LoadByteSource::CA8   => self.bus.read_byte(0xFF00 | self.reg.c as u16),
                            LoadByteSource::A16  => {
                                // pc+1 holds LSB, pc+2 holds MSB
                                let addr = self.bus.read_byte(self.pc + 1) as u16 | ((self.bus.read_byte(self.pc + 2) as u16) << 8);
                                self.bus.read_byte(addr)
                            },
                            LoadByteSource::IMM8 => self.bus.read_byte(self.pc + 1),
                        };

                        match dest {
                            LoadByteDestination::A   => self.reg.a = source_value,
                            LoadByteDestination::B   => self.reg.b = source_value,
                            LoadByteDestination::C   => self.reg.c = source_value,
                            LoadByteDestination::D   => self.reg.d = source_value,
                            LoadByteDestination::E   => self.reg.e = source_value,
                            LoadByteDestination::H   => self.reg.h = source_value,
                            LoadByteDestination::L   => self.reg.l = source_value,
                            LoadByteDestination::HLI => {
                                self.bus.write_byte(self.reg.get_hl(), source_value);
                                self.reg.set_hl(self.reg.get_hl().wrapping_add(1));
                            },
                            LoadByteDestination::HLD => {
                                self.bus.write_byte(self.reg.get_hl(), source_value);
                                self.reg.set_hl(self.reg.get_hl().wrapping_sub(1));
                            },
                            LoadByteDestination::BC  => self.bus.write_byte(self.reg.get_bc(), source_value),
                            LoadByteDestination::DE  => self.bus.write_byte(self.reg.get_de(), source_value),
                            LoadByteDestination::HL  => self.bus.write_byte(self.reg.get_hl(), source_value),
                            LoadByteDestination::A8  => {
                                let addr = 0xFF00 | self.bus.read_byte(self.pc + 1) as u16;
                                self.bus.write_byte(addr, source_value)
                            },
                            LoadByteDestination::CA8  => {
                                let addr = 0xFF00 | self.reg.c as u16;
                                self.bus.write_byte(addr, source_value)
                            },
                            LoadByteDestination::A16 => {
                                // pc+1 holds LSB, pc+2 holds MSB
                                let addr = self.bus.read_byte(self.pc + 1) as u16 | (self.bus.read_byte(self.pc + 2) as u16) << 8;
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

            Instruction::CALL(condition) => {
                if match condition {
                    ControlCondition::NZ   => !self.reg.f.zero,
                    ControlCondition::NC   => !self.reg.f.carry,
                    ControlCondition::Z    => self.reg.f.zero,
                    ControlCondition::C    => self.reg.f.carry,
                    ControlCondition::NONE => true,
                    _ => panic!("Unsupported CALL ControlCondition"),
                }
                {
                    // calling takes 12 extra cycles
                    extra_cycles = 12;

                    // save pc and return new pc
                    let return_pc = self.pc + 3;
                    self.push((return_pc >> 8) as u8);
                    self.push((return_pc & 0xFF) as u8);
                    (self.bus.read_byte(self.pc + 1) as u16) | ((self.bus.read_byte(self.pc + 2) as u16) << 8)
                }
                else {
                    self.pc + 3
                }
            }

            Instruction::RET(condition) => {
                if match condition {
                    ControlCondition::NZ      => !self.reg.f.zero,
                    ControlCondition::NC      => !self.reg.f.carry,
                    ControlCondition::Z       => self.reg.f.zero,
                    ControlCondition::C       => self.reg.f.carry,
                    ControlCondition::NONE    => true,
                    ControlCondition::NONEEI => {
                        // TODO: unsure that interrupts should be enabled this early during RETI
                        self.interrupt_enable = true;
                        true
                    }
                }
                {
                    // returning takes 12 extra cycles
                    extra_cycles = 12;

                    // return to the pc on stack
                    let pch = self.pop();
                    let pcl = self.pop();
                    ((pch as u16) << 8) | (pcl as u16)
                }
                else {
                    self.pc + 1
                }
            }

            Instruction::RST(hvalue) => {
                let value = match hvalue {
                    RstValue::H00 => 0x00,
                    RstValue::H10 => 0x10,
                    RstValue::H20 => 0x20,
                    RstValue::H30 => 0x30,
                    RstValue::H08 => 0x08,
                    RstValue::H18 => 0x18,
                    RstValue::H28 => 0x28,
                    RstValue::H38 => 0x38,
                };
                self.push((self.pc >> 8) as u8);
                self.push((self.pc & 0xFF) as u8);
                value as u16
            }

            Instruction::JP(condition, addr_type) => {
                if match condition {
                    ControlCondition::NZ   => !self.reg.f.zero,
                    ControlCondition::NC   => !self.reg.f.carry,
                    ControlCondition::Z    => self.reg.f.zero,
                    ControlCondition::C    => self.reg.f.carry,
                    ControlCondition::NONE => true,
                    _ => panic!("Unsupported JP ControlCondition"),
                }
                {
                    // taking jump takes 4 extra cycles
                    extra_cycles = 4;

                    // return next pc
                    match addr_type {
                        JumpAddr::IMM16 => (self.bus.read_byte(self.pc + 1) as u16) | ((self.bus.read_byte(self.pc + 2) as u16) << 8),
                        JumpAddr::HL    => self.reg.get_hl(),
                        JumpAddr::REL   => self.pc.wrapping_add(self.bus.read_byte(self.pc + 1) as u16),
                    }
                }
                else {
                    // didn't jump, get next pc
                    match addr_type {
                        JumpAddr::IMM16 => self.pc + 3,
                        JumpAddr::HL    => self.pc + 1,
                        JumpAddr::REL   => self.pc + 2,
                    }
                }
            }

            Instruction::NOP => {
                self.pc + 1
            }

            Instruction::STOP => {
                self.is_stopped = true;
                self.pc + 1
            }

            Instruction::HALT => {
                self.is_halted = true;
                self.pc + 1
            }

            Instruction::DI => {
                self.interrupt_enable = false;
                self.pc + 1
            }

            Instruction::EI => {
                self.interrupt_enable = true;
                self.pc + 1
            }

            Instruction::DAA => {
                // Convert A to packed BCD and update flags: Z - 0 C
                let mut carry = false;

                if !self.reg.f.subtract {
                    if self.reg.f.carry || self.reg.a > 0x99 {
                        self.reg.a = self.reg.a.wrapping_add(0x60);
                        carry = true;
                    }
                    if self.reg.f.half_carry || self.reg.a & 0x0f > 0x09 {
                        self.reg.a = self.reg.a.wrapping_add(0x06);
                    }
                } else if self.reg.f.carry {
                    carry = true;
                    self.reg.a = self.reg.a.wrapping_add(if self.reg.f.half_carry { 0x9a } else { 0xa0 });
                } else if self.reg.f.half_carry {
                    self.reg.a = self.reg.a.wrapping_add(0xfa);
                }
                self.reg.f.zero = self.reg.a == 0;
                self.reg.f.half_carry = false;
                self.reg.f.carry = carry;
                self.pc + 1
            }

            Instruction::CPL => {
                // Invert A and update flags: - 1 1 -
                self.reg.a ^= 0xFF;
                self.reg.f.subtract = true;
                self.reg.f.half_carry = true;
                self.pc + 1
            }

            Instruction::CCF => {
                // Invert carry flag and update flags: - 0 0 C
                self.reg.f.subtract = false;
                self.reg.f.half_carry = false;
                self.reg.f.carry = !self.reg.f.carry;
                self.pc + 1
            }

            Instruction::SCF => {
                // Update flags: - 0 0 1
                self.reg.f.subtract = false;
                self.reg.f.half_carry = false;
                self.reg.f.carry = true;
                self.pc + 1
            }
        };

        (next_pc, extra_cycles)
    }

    // Reads and executes instruction at pc
    pub fn step(&mut self) -> u8 {
        let mut instruction_byte = self.bus.read_byte(self.pc);
        let prefixed = instruction_byte == 0xCB;

        if prefixed {
            instruction_byte = self.bus.read_byte(self.pc + 1);
        }

        let (next_pc, extra_cycles) = if let Some(instruction) = Instruction::from_byte(instruction_byte, prefixed) {
            self.execute(instruction)
        } else {
            let description = format!("0x{}{:x}", if prefixed { "CB" } else { "" }, instruction_byte);
            panic!("Unknown instruction {}", description);
        };

        self.pc = next_pc;
        extra_cycles + get_cycle_count(instruction_byte, prefixed)
    }

    // Execute however many instructions fit in a frame
    pub fn frame_step(&mut self) {
        let cycles_per_frame = (self.frequency * self.frame_delay ) / 1000 / 1000;
        let mut current_frame_cycles = 0;

        while current_frame_cycles < cycles_per_frame {
            current_frame_cycles += self.step() as u64;
        }
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
        let (new_value, _) = value.overflowing_add(1);
        self.reg.f.zero = new_value == 0;
        self.reg.f.subtract = false;
        self.reg.f.half_carry = value == 0xF;
        new_value
    }

    // Return value - 1 and update flags: Z 1 H -
    fn dec(&mut self, value: u8) -> u8 {
        let (new_value, _) = value.overflowing_sub(1);
        self.reg.f.zero = new_value == 0;
        self.reg.f.subtract = true;
        self.reg.f.half_carry = value & 0xF == 0;
        new_value
    }

    fn push(&mut self, value: u8) {
        self.bus.write_byte(self.sp - 1, value);
        self.sp -= 1;
    }

    fn pop(&mut self) -> u8 {
        let value = self.bus.read_byte(self.sp);
        self.sp += 1;
        value
    }
}

#[cfg(test)]
mod test_cpu;
