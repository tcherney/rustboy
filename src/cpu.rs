use crate::graphics::Graphics;
use crate::memorybus::MemoryBus;
use crate::registers::FlagsRegister;
use crate::registers::Registers;
use std::{cell::RefCell, rc::Rc};

pub struct CPU {
    registers: Registers,
    pc: u16, // Program Counter
    sp: u16,
    bus: MemoryBus,
    is_halted: bool,
}

#[derive(Debug)]
enum JumpCondition {
    Always,
    Zero,
    NotZero,
    Carry,
    NotCarry,
}

#[derive(Debug)]
enum LoadByteTarget {
    A,
    B,
    C,
    D,
    E,
    H,
    L,
    HLI,
}
#[derive(Debug)]
enum LoadByteSource {
    A,
    B,
    C,
    D,
    E,
    H,
    L,
    D8,
    HLI,
}

#[derive(Debug)]
enum LoadType {
    Byte(LoadByteTarget, LoadByteSource),
}

#[derive(Debug)]
enum StackTarget {
    BC,
    DE,
    HL,
    AF,
}

#[derive(Debug)]
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
    RES(u8, RegisterTarget),
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
            0x00 => Some(Instruction::RLC(RegisterTarget::B)),
            0x01 => Some(Instruction::RLC(RegisterTarget::C)),
            0x02 => Some(Instruction::RLC(RegisterTarget::D)),
            0x03 => Some(Instruction::RLC(RegisterTarget::E)),
            0x04 => Some(Instruction::RLC(RegisterTarget::H)),
            0x05 => Some(Instruction::RLC(RegisterTarget::L)),
            0x06 => Some(Instruction::RLC(RegisterTarget::A)), // RLC (HL)
            0x07 => Some(Instruction::RLC(RegisterTarget::A)), // RLC A
            0x08 => Some(Instruction::RR(RegisterTarget::B)),
            0x09 => Some(Instruction::RR(RegisterTarget::C)),
            0x0A => Some(Instruction::RR(RegisterTarget::D)),
            0x0B => Some(Instruction::RR(RegisterTarget::E)),
            0x0C => Some(Instruction::RR(RegisterTarget::H)),
            0x0D => Some(Instruction::RR(RegisterTarget::L)),
            0x0E => Some(Instruction::RR(RegisterTarget::A)), // RR (HL)
            0x0F => Some(Instruction::RR(RegisterTarget::A)), // RR A
            0x10 => Some(Instruction::RL(RegisterTarget::B)),
            0x11 => Some(Instruction::RL(RegisterTarget::C)),
            0x12 => Some(Instruction::RL(RegisterTarget::D)),
            0x13 => Some(Instruction::RL(RegisterTarget::E)),
            0x14 => Some(Instruction::RL(RegisterTarget::H)),
            0x15 => Some(Instruction::RL(RegisterTarget::L)),
            0x16 => Some(Instruction::RL(RegisterTarget::A)), // RL (HL)
            0x17 => Some(Instruction::RL(RegisterTarget::A)), // RL A
            0x18 => Some(Instruction::RRC(RegisterTarget::B)),
            0x19 => Some(Instruction::RRC(RegisterTarget::C)),
            0x1A => Some(Instruction::RRC(RegisterTarget::D)),
            0x1B => Some(Instruction::RRC(RegisterTarget::E)),
            0x1C => Some(Instruction::RRC(RegisterTarget::H)),
            0x1D => Some(Instruction::RRC(RegisterTarget::L)),
            0x1E => Some(Instruction::RRC(RegisterTarget::A)), // RRC (HL)
            0x1F => Some(Instruction::RRC(RegisterTarget::A)), // RRC A
            0x20 => Some(Instruction::SLA(RegisterTarget::B)),
            0x21 => Some(Instruction::SLA(RegisterTarget::C)),
            0x22 => Some(Instruction::SLA(RegisterTarget::D)),
            0x23 => Some(Instruction::SLA(RegisterTarget::E)),
            0x24 => Some(Instruction::SLA(RegisterTarget::H)),
            0x25 => Some(Instruction::SLA(RegisterTarget::L)),
            0x26 => Some(Instruction::SLA(RegisterTarget::A)), // SLA (HL)
            0x27 => Some(Instruction::SLA(RegisterTarget::A)), // SLA A
            0x28 => Some(Instruction::SRA(RegisterTarget::B)),
            0x29 => Some(Instruction::SRA(RegisterTarget::C)),
            0x2A => Some(Instruction::SRA(RegisterTarget::D)),
            0x2B => Some(Instruction::SRA(RegisterTarget::E)),
            0x2C => Some(Instruction::SRA(RegisterTarget::H)),
            0x2D => Some(Instruction::SRA(RegisterTarget::L)),
            0x2E => Some(Instruction::SRA(RegisterTarget::A)), // SRA (HL)
            0x2F => Some(Instruction::SRA(RegisterTarget::A)), // SRA A
            0x30 => Some(Instruction::SWAP(RegisterTarget::B)),
            0x31 => Some(Instruction::SWAP(RegisterTarget::C)),
            0x32 => Some(Instruction::SWAP(RegisterTarget::D)),
            0x33 => Some(Instruction::SWAP(RegisterTarget::E)),
            0x34 => Some(Instruction::SWAP(RegisterTarget::H)),
            0x35 => Some(Instruction::SWAP(RegisterTarget::L)),
            0x36 => Some(Instruction::SWAP(RegisterTarget::A)), // SWAP (HL)
            0x37 => Some(Instruction::SWAP(RegisterTarget::A)), // SWAP A
            0x38 => Some(Instruction::SRL(RegisterTarget::B)),
            0x39 => Some(Instruction::SRL(RegisterTarget::C)),
            0x3A => Some(Instruction::SRL(RegisterTarget::D)),
            0x3B => Some(Instruction::SRL(RegisterTarget::E)),
            0x3C => Some(Instruction::SRL(RegisterTarget::H)),
            0x3D => Some(Instruction::SRL(RegisterTarget::L)),
            0x3E => Some(Instruction::SRL(RegisterTarget::A)), // SRL (HL)
            0x3F => Some(Instruction::SRL(RegisterTarget::A)), // SRL A
            0x40..=0x7F => {
                let bit = (byte - 0x40) / 8;
                let reg_code = (byte - 0x40) % 8;
                let target = match reg_code {
                    0 => RegisterTarget::B,
                    1 => RegisterTarget::C,
                    2 => RegisterTarget::D,
                    3 => RegisterTarget::E,
                    4 => RegisterTarget::H,
                    5 => RegisterTarget::L,
                    6 => RegisterTarget::A, // BIT (HL)
                    7 => RegisterTarget::A, // BIT A
                    _ => unreachable!(),
                };
                Some(Instruction::BIT(bit, target))
            }
            0x80..=0xBF => {
                let bit = (byte - 0x80) / 8;
                let reg_code = (byte - 0x80) % 8;
                let target = match reg_code {
                    0 => RegisterTarget::B,
                    1 => RegisterTarget::C,
                    2 => RegisterTarget::D,
                    3 => RegisterTarget::E,
                    4 => RegisterTarget::H,
                    5 => RegisterTarget::L,
                    6 => RegisterTarget::A, // SET (HL)
                    7 => RegisterTarget::A, // SET A
                    _ => unreachable!(),
                };
                Some(Instruction::SET(bit, target))
            }
            0xC0..=0xFF => {
                let bit = (byte - 0xC0) / 8;
                let reg_code = (byte - 0xC0) % 8;
                let target = match reg_code {
                    0 => RegisterTarget::B,
                    1 => RegisterTarget::C,
                    2 => RegisterTarget::D,
                    3 => RegisterTarget::E,
                    4 => RegisterTarget::H,
                    5 => RegisterTarget::L,
                    6 => RegisterTarget::A, // RES (HL)
                    7 => RegisterTarget::A, // RES A
                    _ => unreachable!(),
                };
                Some(Instruction::RES(bit, target))
            }
            _ => None,
        }
    }
    fn from_byte_not_prefix(byte: u8) -> Option<Self> {
        match byte {
            0x00 => Some(Instruction::NOP),
            0x76 => Some(Instruction::HALT),
            0xC3 => Some(Instruction::JP(JumpCondition::Always)),
            0xC2 => Some(Instruction::JP(JumpCondition::NotZero)),
            0xCA => Some(Instruction::JP(JumpCondition::Zero)),
            0xD2 => Some(Instruction::JP(JumpCondition::NotCarry)),
            0xDA => Some(Instruction::JP(JumpCondition::Carry)),
            0xCD => Some(Instruction::CALL(JumpCondition::Always)),
            0xC4 => Some(Instruction::CALL(JumpCondition::NotZero)),
            0xCC => Some(Instruction::CALL(JumpCondition::Zero)),
            0xD4 => Some(Instruction::CALL(JumpCondition::NotCarry)),
            0xDC => Some(Instruction::CALL(JumpCondition::Carry)),
            0xC9 => Some(Instruction::RET(JumpCondition::Always)),
            0xC0 => Some(Instruction::RET(JumpCondition::NotZero)),
            0xC8 => Some(Instruction::RET(JumpCondition::Zero)),
            0xD0 => Some(Instruction::RET(JumpCondition::NotCarry)),
            0xD8 => Some(Instruction::RET(JumpCondition::Carry)),
            0xF5 => Some(Instruction::PUSH(StackTarget::AF)),
            0xC5 => Some(Instruction::PUSH(StackTarget::BC)),
            0xD5 => Some(Instruction::PUSH(StackTarget::DE)),
            0xE5 => Some(Instruction::PUSH(StackTarget::HL)),
            0xF1 => Some(Instruction::POP(StackTarget::AF)),
            0xC1 => Some(Instruction::POP(StackTarget::BC)),
            0xD1 => Some(Instruction::POP(StackTarget::DE)),
            0xE1 => Some(Instruction::POP(StackTarget::HL)),
            0x3E => Some(Instruction::LD(LoadType::Byte(
                LoadByteTarget::A,
                LoadByteSource::D8,
            ))),
            0x06 => Some(Instruction::LD(LoadType::Byte(
                LoadByteTarget::B,
                LoadByteSource::D8,
            ))),
            0x0E => Some(Instruction::LD(LoadType::Byte(
                LoadByteTarget::C,
                LoadByteSource::D8,
            ))),
            0x16 => Some(Instruction::LD(LoadType::Byte(
                LoadByteTarget::D,
                LoadByteSource::D8,
            ))),
            0x1E => Some(Instruction::LD(LoadType::Byte(
                LoadByteTarget::E,
                LoadByteSource::D8,
            ))),
            0x26 => Some(Instruction::LD(LoadType::Byte(
                LoadByteTarget::H,
                LoadByteSource::D8,
            ))),
            0x2E => Some(Instruction::LD(LoadType::Byte(
                LoadByteTarget::L,
                LoadByteSource::D8,
            ))),
            0x78 => Some(Instruction::LD(LoadType::Byte(
                LoadByteTarget::A,
                LoadByteSource::B,
            ))),
            0x79 => Some(Instruction::LD(LoadType::Byte(
                LoadByteTarget::A,
                LoadByteSource::C,
            ))),
            0x7A => Some(Instruction::LD(LoadType::Byte(
                LoadByteTarget::A,
                LoadByteSource::D,
            ))),
            0x7B => Some(Instruction::LD(LoadType::Byte(
                LoadByteTarget::A,
                LoadByteSource::E,
            ))),
            0x7C => Some(Instruction::LD(LoadType::Byte(
                LoadByteTarget::A,
                LoadByteSource::H,
            ))),
            0x7D => Some(Instruction::LD(LoadType::Byte(
                LoadByteTarget::A,
                LoadByteSource::L,
            ))),
            0x7E => Some(Instruction::LD(LoadType::Byte(
                LoadByteTarget::A,
                LoadByteSource::HLI,
            ))),
            0x40 => Some(Instruction::LD(LoadType::Byte(
                LoadByteTarget::B,
                LoadByteSource::B,
            ))),
            0x41 => Some(Instruction::LD(LoadType::Byte(
                LoadByteTarget::B,
                LoadByteSource::C,
            ))),
            0x42 => Some(Instruction::LD(LoadType::Byte(
                LoadByteTarget::B,
                LoadByteSource::D,
            ))),
            0x43 => Some(Instruction::LD(LoadType::Byte(
                LoadByteTarget::B,
                LoadByteSource::E,
            ))),
            0x44 => Some(Instruction::LD(LoadType::Byte(
                LoadByteTarget::B,
                LoadByteSource::H,
            ))),
            0x45 => Some(Instruction::LD(LoadType::Byte(
                LoadByteTarget::B,
                LoadByteSource::L,
            ))),
            0x46 => Some(Instruction::LD(LoadType::Byte(
                LoadByteTarget::B,
                LoadByteSource::HLI,
            ))),
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

#[derive(Debug)]
enum RegisterTarget {
    A,
    B,
    C,
    D,
    E,
    H,
    L,
    BC,
    DE,
    HL,
    AF,
    SP,
    D8,
}

impl CPU {
    pub fn new(graphics: Rc<RefCell<Graphics>>) -> Self {
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
    pub fn step(&mut self) {
        if self.is_halted {
            return;
        }
        let mut opcode = self.bus.read_byte(self.pc);
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
    pub fn boot(&mut self) {
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
                    RegisterTarget::HL => {
                        let address = self.registers.get_hl();
                        let value = self.bus.read_byte(address);
                        let result = self.add(value);
                        self.registers.a = result;
                        self.pc.wrapping_add(1)
                    }
                    //TODO may need to handle D8 differently, but for now just add them to the match
                    RegisterTarget::D8 => {
                        let immediate_value = self.bus.read_byte(self.pc + 1);
                        let result = self.add(immediate_value);
                        self.registers.a = result;
                        self.pc.wrapping_add(2)
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
                    RegisterTarget::DE => {
                        let value = self.registers.get_de();
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
                    RegisterTarget::HL => {
                        let value = self.registers.get_hl();
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
                    RegisterTarget::SP => {
                        let value = self.sp;
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
                        panic!("Unknown target: {:#?}", target);
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
                    let low: u16 = self.bus.read_byte(self.sp).into();
                    self.sp = self.sp.wrapping_add(1);
                    let high: u16 = self.bus.read_byte(self.sp).into();
                    self.sp = self.sp.wrapping_add(1);
                    let return_address: u16 = (high << 8) | low;
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
                let low: u16 = self.bus.read_byte(self.sp).into();
                self.sp = self.sp.wrapping_add(1);
                let high: u16 = self.bus.read_byte(self.sp).into();
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
                panic!("Unknown instruction: {:#?}", instruction);
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
