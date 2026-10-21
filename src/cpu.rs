use crate::graphics::Graphics;
use crate::registerqs::Registers;
use crate::memorybus::MemoryBus;

struct CPU {
    registers: Registers,
    pc: u16,              // Program Counter
    sp: u16,
    bus: MemoryBus,
    is_halted: bool,
}

enum JumpCondition {
    Always,
    Zero,
    NotZero,
    Carry,
    NotCarry,
}

enum LoadByteTarget {
    A, B, C, D, E, H, L, HLI
}
enum LoadByteSource {
    A, B, C, D, E, H, L, D8, HLI
}
enum LoadType {
    Byte(LoadByteTarget, LoadByteSource),
}

enum StackTarget {
    BC, DE, HL, AF
}

enum Instruction {
    NOP,
    ADD(RegisterTarget),
    SUB(RegisterTarget),
    JP(JumpCondition),
    ADDHL(RegisterTarget),
    ADC(RegisterTarget),
    SBC(RegisterTarget),
    AND(RegisterTarget),
    OR(RegisterTarget),
    XOR(RegisterTarget),
    CP(RegisterTarget),
    INC(RegisterTarget),
    DEC(RegisterTarget),
    CCF,
    SCF,
    RRA,
    RLA,
    RRCA,
    RRLA,
    CPL,
    BIT(u8, RegisterTarget),
    SET(u8, RegisterTarget),
    SRL(RegisterTarget),
    RR(RegisterTarget),
    RL(RegisterTarget),
    RRC(RegisterTarget),
    RLC(RegisterTarget),
    SRA(RegisterTarget),
    SLA(RegisterTarget),
    SWAP(RegisterTarget),
    LD(LoadType),
    PUSH(StackTarget),
    POP(StackTarget),
    CALL(JumpCondition),
    RET(JumpCondition),
    HALT,
}

//TODO verify this, prefix will def have to be fixed
impl Instruction {
    fn from_byte(byte: u8, prefix: bool) -> Option<Self> {
        if prefix {
            Self::from_byte_prefix(byte)
        } else {
            Self::from_byte_not_prefix(byte)
        }
    }
    fn from_byte_prefix(byte: u8) -> Option<Self> {
        match byte {
            0x00 => Some(Instruction::NOP),
            0x80 => Some(Instruction::ADD(RegisterTarget::B)),
            0x81 => Some(Instruction::ADD(RegisterTarget::C)),
            0x82 => Some(Instruction::ADD(RegisterTarget::D)),
            0x83 => Some(Instruction::ADD(RegisterTarget::E)),
            0x84 => Some(Instruction::ADD(RegisterTarget::H)),
            0x85 => Some(Instruction::ADD(RegisterTarget::L)),
            0x86 => Some(Instruction::ADD(RegisterTarget::A)), // ADD A, (HL)
            0x87 => Some(Instruction::ADD(RegisterTarget::A)), // ADD A, A
            0x90 => Some(Instruction::SUB(RegisterTarget::B)),
            0x91 => Some(Instruction::SUB(RegisterTarget::C)),
            0x92 => Some(Instruction::SUB(RegisterTarget::D)),
            0x93 => Some(Instruction::SUB(RegisterTarget::E)),
            0x94 => Some(Instruction::SUB(RegisterTarget::H)),
            0x95 => Some(Instruction::SUB(RegisterTarget::L)),
            0x96 => Some(Instruction::SUB(RegisterTarget::A)), // SUB (HL)
            0x97 => Some(Instruction::SUB(RegisterTarget::A)), // SUB A
            _ => None,
        }
    }
    fn from_byte_not_prefix(byte: u8) -> Option<Self> {
        match byte {
            0x00 => Some(Instruction::NOP),
            0x80 => Some(Instruction::ADD(RegisterTarget::B)),
            0x81 => Some(Instruction::ADD(RegisterTarget::C)),
            0x82 => Some(Instruction::ADD(RegisterTarget::D)),
            0x83 => Some(Instruction::ADD(RegisterTarget::E)),
            0x84 => Some(Instruction::ADD(RegisterTarget::H)),
            0x85 => Some(Instruction::ADD(RegisterTarget::L)),
            0x86 => Some(Instruction::ADD(RegisterTarget::A)), // ADD A, (HL)
            0x87 => Some(Instruction::ADD(RegisterTarget::A)), // ADD A, A
            0x90 => Some(Instruction::SUB(RegisterTarget::B)),
            0x91 => Some(Instruction::SUB(RegisterTarget::C)),
            0x92 => Some(Instruction::SUB(RegisterTarget::D)),
            0x93 => Some(Instruction::SUB(RegisterTarget::E)),
            0x94 => Some(Instruction::SUB(RegisterTarget::H)),
            0x95 => Some(Instruction::SUB(RegisterTarget::L)),
            0x96 => Some(Instruction::SUB(RegisterTarget::A)), // SUB (HL)
            0x97 => Some(Instruction::SUB(RegisterTarget::A)), // SUB A
            _ => None,
        }
    }
}

enum RegisterTarget {
    A,
    B,
    C,
    D,
    E,
    H,
    L,
}

impl CPU {
    fn new(graphics: Graphics) -> Self {
        CPU {
            registers: Registers {
                a: 0,
                b: 0,
                c: 0,
                d: 0,
                e: 0,
                f: FlagsRegister {
                    zero: false,
                    subtract: false,
                    half_carry: false,
                    carry: false,
                },
                h: 0,
                l: 0,
            },
            pc: 0,
            sp: 0,
            is_halted: false,
            bus: MemoryBus::new(graphics),
        }
    }
    fn step(&mut self) {
        if self.is_halted {
            return;
        }
        let opcode = self.bus.read_byte(self.pc);
        let prefix = opcode == 0xCB;
        if prefix {
            opcode = self.bus.read_byte(self.pc + 1);
        }
        let next_pc = if let Some(instruction) = Instruction::from_byte(opcode, prefix) {
            self.execute(instruction)
        } else {
            panic!("Unknown opcode: {:#X}", opcode);
        };
        self.pc = next_pc;
    }
    fn boot(&mut self) {
        // Boot sequence implementation
        // Initialize registers, memory, etc.
    }
    fn execute(&mut self, instruction: Instruction) -> u16 {
        match instruction {
            Instruction::NOP => {
                // NOP
                self.pc.wrapping_add(1)
            }
            Instruction::HALT => {
                self.is_halted = true;
                self.pc.wrapping_add(1)
            }
            Instruction::ADC(target) => {
                // ADC implementation
                match target {
                    RegisterTarget::A => {
                        let value = self.registers.a;
                        let result = self.add(value);
                        self.registers.a = result;
                        self.pc.wrapping_add(1)
                    }
                    _ => {
                        // Handle other register targets
                        self.pc
                    }
                }
            }
            Instruction::ADDHL(target) => {
                match target {
                    RegisterTarget::BC => {
                        let value = self.registers.get_bc();
                        let hl = self.registers.get_hl();
                        let result = hl.wrapping_add(value);
                        self.registers.set_hl(result);
                        // Update flags
                        self.registers.f.zero = false;
                        self.registers.f.subtract = false;
                        self.registers.f.half_carry = (hl & 0x0FFF) + (value & 0x0FFF) > 0x0FFF;
                        self.registers.f.carry = (hl as u32) + (value as u32) > 0xFFFF;
                        self.pc.wrapping_add(1)
                    }
                    _ => {
                        // Handle other register targets
                        self.pc
                    }
                }
            }
            Instruction::AND(target) => {
                match target {
                    RegisterTarget::A => {
                        let value = self.registers.a;
                        self.registers.a &= value;
                        // Update flags
                        self.registers.f.zero = self.registers.a == 0;
                        self.registers.f.subtract = false;
                        self.registers.f.half_carry = true;
                        self.registers.f.carry = false;
                        self.pc.wrapping_add(1)
                    }
                    _ => {
                        // Handle other register targets
                        self.pc
                    }
                }
            }
            Instruction::JP(condition) => {
                let jump_condition = match condition {
                    JumpCondition::Always => true,
                    JumpCondition::Zero => self.registers.f.zero,
                    JumpCondition::NotZero => !self.registers.f.zero,
                    JumpCondition::Carry => self.registers.f.carry,
                    JumpCondition::NotCarry => !self.registers.f.carry,
                };
                self.jump(jump_condition)
            }
            Instruction::CALL(condition) => {
                let call_condition = match condition {
                    JumpCondition::Always => true,
                    JumpCondition::Zero => self.registers.f.zero,
                    JumpCondition::NotZero => !self.registers.f.zero,
                    JumpCondition::Carry => self.registers.f.carry,
                    JumpCondition::NotCarry => !self.registers.f.carry,
                };
                if call_condition {
                    let low = self.bus.read_byte(self.pc + 1) as u16;
                    let high = self.bus.read_byte(self.pc + 2) as u16;
                    let address = (high << 8) | low;
                    // Push current PC + 3 onto the stack
                    self.sp = self.sp.wrapping_sub(1);
                    self.bus.write_byte(self.sp, ((self.pc + 3) >> 8) as u8); // High byte
                    self.sp = self.sp.wrapping_sub(1);
                    self.bus.write_byte(self.sp, ((self.pc + 3) & 0xFF) as u8); // Low byte
                    address
                } else {
                    self.pc.wrapping_add(3)
                }
            }
            Instruction::RET(condition) => {
                let ret_condition = match condition {
                    JumpCondition::Always => true,
                    JumpCondition::Zero => self.registers.f.zero,
                    JumpCondition::NotZero => !self.registers.f.zero,
                    JumpCondition::Carry => self.registers.f.carry,
                    JumpCondition::NotCarry => !self.registers.f.carry,
                };
                if ret_condition {
                    // Pop the return address from the stack
                    let low = self.bus.read_byte(self.sp);
                    self.sp = self.sp.wrapping_add(1);
                    let high = self.bus.read_byte(self.sp);
                    self.sp = self.sp.wrapping_add(1);
                    let return_address = (high << 8) | low;
                    return_address
                } else {
                    self.pc.wrapping_add(1)
                }
            }  
            Instruction::PUSH(target) => {
                let value = match target {
                    StackTarget::BC => self.registers.get_bc(),
                    StackTarget::DE => self.registers.get_de(),
                    StackTarget::HL => self.registers.get_hl(),
                    StackTarget::AF => self.registers.get_af(),
                };
                self.sp = self.sp.wrapping_sub(1);
                self.bus.write_byte(self.sp, (value >> 8) as u8);
                self.sp = self.sp.wrapping_sub(1);
                self.bus.write_byte(self.sp, (value & 0xFF) as u8);
                self.pc.wrapping_add(1)
            }
            Instruction::POP(target) => {
                let low = self.bus.read_byte(self.sp);
                self.sp = self.sp.wrapping_add(1);
                let high = self.bus.read_byte(self.sp);
                self.sp = self.sp.wrapping_add(1);
                let value = (high << 8) | low;
                match target {
                    StackTarget::BC => self.registers.set_bc(value),
                    StackTarget::DE => self.registers.set_de(value),
                    StackTarget::HL => self.registers.set_hl(value),
                    StackTarget::AF => self.registers.set_af(value),
                }
                self.pc.wrapping_add(1)
            }
            Instruction::LD(load_type) => {
                match load_type {
                    LoadType::Byte(target, source) => {
                        let value = match source {
                            LoadByteSource::A => self.registers.a,
                            LoadByteSource::B => self.registers.b,
                            LoadByteSource::C => self.registers.c,
                            LoadByteSource::D => self.registers.d,
                            LoadByteSource::E => self.registers.e,
                            LoadByteSource::H => self.registers.h,
                            LoadByteSource::L => self.registers.l,
                            LoadByteSource::D8 => {
                                let immediate_value = self.bus.read_byte(self.pc + 1);
                                immediate_value
                            }
                            LoadByteSource::HLI => {
                                let address = self.registers.get_hl();
                                self.bus.read_byte(address)
                            }
                        };
                        match target {
                            LoadByteTarget::A => self.registers.a = value,
                            LoadByteTarget::B => self.registers.b = value,
                            LoadByteTarget::C => self.registers.c = value,
                            LoadByteTarget::D => self.registers.d = value,
                            LoadByteTarget::E => self.registers.e = value,
                            LoadByteTarget::H => self.registers.h = value,
                            LoadByteTarget::L => self.registers.l = value,
                            LoadByteTarget::HLI => {
                                let address = self.registers.get_hl();
                                self.bus.write_byte(address, value);
                            }
                        }
                        if let LoadByteSource::D8 = source {
                            // If the source was an immediate value, we need to increment PC by 2
                            return self.pc.wrapping_add(2);
                        } else {
                            // Otherwise, increment PC by 1
                            return self.pc.wrapping_add(1);
                        }
                    }
                }
            }   
            Instruction::ADD(target) => {
                // ADD implementation
                match target {
                    RegisterTarget::A => {
                        let value = self.registers.a;
                        let result = self.add(value);
                        self.registers.a = result;
                        self.pc.wrapping_add(1)
                    }
                    RegisterTarget::B => {
                        let value = self.registers.b;
                        let result = self.add(value);
                        self.registers.a = result;
                        self.pc.wrapping_add(1)
                    }
                    RegisterTarget::C => {
                        let value = self.registers.c;
                        let result = self.add(value);
                        self.registers.a = result;
                        self.pc.wrapping_add(1)
                    }
                    RegisterTarget::D => {
                        let value = self.registers.d;
                        let result = self.add(value);
                        self.registers.a = result;
                        self.pc.wrapping_add(1)
                    }
                    RegisterTarget::E => {
                        let value = self.registers.e;
                        let result = self.add(value);
                        self.registers.a = result;
                        self.pc.wrapping_add(1)
                    }
                    RegisterTarget::H => {
                        let value = self.registers.h;
                        let result = self.add(value);
                        self.registers.a = result;
                        self.pc.wrapping_add(1)
                    }
                    RegisterTarget::L => {
                        let value = self.registers.l;
                        let result = self.add(value);
                        self.registers.a = result;
                        self.pc.wrapping_add(1)
                    }
                    _ => {
                        // Handle other register targets
                        self.pc
                    }
                }
            }
            Instruction::SUB(target) => {
                // SUB implementation
                match target {
                    RegisterTarget::A => {
                        let value = self.registers.a;
                        let result = self.sub(value);
                        self.registers.a = result;
                        self.pc.wrapping_add(1)
                    }
                    RegisterTarget::B => {
                        let value = self.registers.b;
                        let result = self.sub(value);
                        self.registers.a = result;
                        self.pc.wrapping_add(1)
                    }
                    RegisterTarget::C => {
                        let value = self.registers.c;
                        let result = self.sub(value);
                        self.registers.a = result;
                        self.pc.wrapping_add(1)
                    }
                    RegisterTarget::D => {
                        let value = self.registers.d;
                        let result = self.sub(value);
                        self.registers.a = result;
                        self.pc.wrapping_add(1)
                    }
                    RegisterTarget::E => {
                        let value = self.registers.e;
                        let result = self.sub(value);
                        self.registers.a = result;
                        self.pc.wrapping_add(1)
                    }
                    RegisterTarget::H => {
                        let value = self.registers.h;
                        let result = self.sub(value);
                        self.registers.a = result;
                        self.pc.wrapping_add(1)
                    }
                    RegisterTarget::L => {
                        let value = self.registers.l;
                        let result = self.sub(value);
                        self.registers.a = result;
                        self.pc.wrapping_add(1)
                    }
                    _ => {
                        // Handle other register targets
                        self.pc
                    }
                }
            }
            _ => {
                panic!("Unknown instruction: {:#X}", instruction);
                self.pc
            }
        }
    }

    fn add(&mut self, value: u8) -> u8 {
        let (result, did_overflow) = self.registers.a.overflowing_add(value);
        self.registers.a = result;
        // Update flags
        self.registers.f.zero = result == 0;
        self.registers.f.subtract = false;
        self.registers.f.half_carry = (self.registers.a & 0x0F) + (value & 0x0F) > 0x0F;
        self.registers.f.carry = did_overflow;
        result
    }

    fn sub(&mut self, value: u8) -> u8 {
        let (result, did_underflow) = self.registers.a.overflowing_sub(value);
        self.registers.a = result;
        // Update flags
        self.registers.f.zero = result == 0;
        self.registers.f.subtract = true;
        self.registers.f.half_carry = (self.registers.a & 0x0F) < (value & 0x0F);
        self.registers.f.carry = did_underflow;
        result
    }

    fn jump(&mut self, condition: bool) -> u16 {
        if condition {
            let low = self.bus.read_byte(self.pc + 1) as u16;
            let high = self.bus.read_byte(self.pc + 2) as u16;
            let address = (high << 8) | low;
            address
        } else {
            self.pc.wrapping_add(3)
        }
    }
}
