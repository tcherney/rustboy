use crate::cpu::CPU;
use crate::graphics::Graphics;

struct Rustboy {
    cpu: CPU,
    graphics: Graphics,
}

impl Rustboy {
    fn new() -> Self {
        Rustboy {
            cpu: CPU::new(),
            graphics: Graphics::new(),
        }
    }

    fn run(&mut self) {
        self.cpu.boot();
        loop {
            self.cpu.step();
            self.graphics.update();
        }
    }
}