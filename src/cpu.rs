use crate::registers::Registers;

struct CPU {
    registers: Registers,
    pc: u16,              // Program Counter
    memory: [u8; 0xFFFF], // 64KB of memory
}

enum JumpCondition {
    Always,
    Zero,
    NotZero,
    Carry,
    NotCarry,
}

enum Instruction {
    NOP,
    ADD(RegisterTarget),
    SUB(RegisterTarget),
    JP(JumpCondition),
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
    fn new() -> Self {
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
            memory: [0; 0x10000],
        }
    }
    fn step(&mut self) {
        let opcode = self.memory[self.pc as usize];
        let prefix = opcode == 0xCB;
        if prefix {
            opcode = self.memory[(self.pc + 1) as usize];
        }
        let next_pc = if let Some(instruction) = Instruction::from_byte(opcode) {
            self.execute(instruction);
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
            let low = self.memory[(self.pc + 1) as usize] as u16;
            let high = self.memory[(self.pc + 2) as usize] as u16;
            let address = (high << 8) | low;
            address
        } else {
            self.pc.wrapping_add(3)
        }
    }
}
