use std::{cell::RefCell, rc::Rc};

use crate::graphics::{self, Graphics};
pub struct MemoryBus {
    memory: [u8; 0xFFFF], // 64KB of memory
    graphics: Rc<RefCell<Graphics>>,
}

impl MemoryBus {
    pub fn new(graphics: Rc<RefCell<Graphics>>) -> Self {
        MemoryBus {
            memory: [0; 0xFFFF],
            graphics,
        }
    }
    pub fn write_byte(&mut self, address: u16, value: u8) {
        if address >= graphics::VRAM_BEGIN as u16 && address < graphics::VRAM_END as u16 {
            self.graphics.borrow_mut().write_vram(address, value);
        } else {
            self.memory[address as usize] = value;
        }
    }
    pub fn read_byte(&self, address: u16) -> u8 {
        if address >= graphics::VRAM_BEGIN as u16 && address < graphics::VRAM_END as u16 {
            self.graphics.borrow().read_vram(address)
        } else {
            self.memory[address as usize]
        }
    }
    pub fn write_word(&mut self, address: u16, value: u16) {
        if address >= graphics::VRAM_BEGIN as u16 && address < graphics::VRAM_END as u16 {
            let mut graphics = self.graphics.borrow_mut();
            graphics.write_vram(address, (value & 0xFF) as u8);
            graphics.write_vram(address + 1, (value >> 8) as u8);
        } else {
            self.memory[address as usize] = (value & 0xFF) as u8;
            self.memory[(address + 1) as usize] = (value >> 8) as u8;
        }
    }
    pub fn read_word(&self, address: u16) -> u16 {
        if address >= graphics::VRAM_BEGIN as u16 && address < graphics::VRAM_END as u16 {
            let graphics = self.graphics.borrow();
            let low = graphics.read_vram(address) as u16;
            let high = graphics.read_vram(address + 1) as u16;
            (high << 8) | low
        } else {
            let low = self.memory[address as usize] as u16;
            let high = self.memory[(address + 1) as usize] as u16;
            (high << 8) | low
        }
    }
}
