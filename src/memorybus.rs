use crate::graphics::Graphics;
use types::graphics;

struct MemoryBus {
    memory: [u8; 0xFFFF], // 64KB of memory
    graphics: Graphics,
}

impl MemoryBus {
    fn new(graphics: Graphics) -> Self {
        MemoryBus {
            memory: [0; 0xFFFF],
            graphics: graphics,
        }
    }
    fn write_byte(&mut self, address: u16, value: u8) {
        if address >= graphics::VRAM_BEGIN as u16 && address < graphics::VRAM_END as u16 {
            self.graphics.write_vram(address, value);
        }
        else {
        self.memory[address as usize] = value;
        }
    }
    fn read_byte(&self, address: u16) -> u8 {
        if address >= graphics::VRAM_BEGIN as u16 && address < graphics::VRAM_END as u16 {
            self.graphics.read_vram(address)
        } else {
            self.memory[address as usize]
        }
    }
    fn write_word(&mut self, address: u16, value: u16) {
        if address >= graphics::VRAM_BEGIN as u16 && address < graphics::VRAM_END as u16 {
            self.graphics.write_vram(address, (value & 0xFF) as u8);
            self.graphics.write_vram(address + 1, (value >> 8) as u8);
        }
        else {      
            self.memory[address as usize] = (value & 0xFF) as u8;
            self.memory[(address + 1) as usize] = (value >> 8) as u8;
        }
    }
    fn read_word(&self, address: u16) -> u16 {
        if address >= graphics::VRAM_BEGIN as u16 && address < graphics::VRAM_END as u16 {
            let low = self.graphics.read_vram(address) as u16;
            let high = self.graphics.read_vram(address + 1) as u16;
            (high << 8) | low
        } else {
            let low = self.memory[address as usize] as u16;
            let high = self.memory[(address + 1) as usize] as u16;
            (high << 8) | low
        } 
    }
}