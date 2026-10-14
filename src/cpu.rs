use crate::registers::Registers;

struct CPU {
    registers: Registers,
}

enum Instruction {
    NOP,
    ADD(RegisterTarget), // Other instructions can be added here
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
        }
    }
    fn execute(&mut self, instruction: Instruction) {
        match instruction {
            Instruction::NOP => {
                // NOP
            }
            Instruction::ADD(target) => {
                // ADD implementation
                match target {
                    RegisterTarget::A => {
                        let value = self.registers.a;
                        let result = self.add(value);
                        self.registers.a = result;
                    }
                    RegisterTarget::B => {
                        let value = self.registers.b;
                        let result = self.add(value);
                        self.registers.a = result;
                    }
                    RegisterTarget::C => {
                        let value = self.registers.c;
                        let result = self.add(value);
                        self.registers.a = result;
                    }
                    RegisterTarget::D => {
                        let value = self.registers.d;
                        let result = self.add(value);
                        self.registers.a = result;
                    }
                    RegisterTarget::E => {
                        let value = self.registers.e;
                        let result = self.add(value);
                        self.registers.a = result;
                    }
                    RegisterTarget::H => {
                        let value = self.registers.h;
                        let result = self.add(value);
                        self.registers.a = result;
                    }
                    RegisterTarget::L => {
                        let value = self.registers.l;
                        let result = self.add(value);
                        self.registers.a = result;
                    }
                    _ => {
                        // Handle other register targets
                    }
                }
            }
            _ => {
                panic!("Unknown instruction: {:#X}", instruction);
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
}
