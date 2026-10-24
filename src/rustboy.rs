use std::{cell::RefCell, rc::Rc};

use crate::cpu::CPU;
use crate::graphics::Graphics;

pub struct Rustboy {
    cpu: CPU,
    graphics: Rc<RefCell<Graphics>>,
}

impl Rustboy {
    pub fn new() -> Self {
        let graphics = Rc::new(RefCell::new(Graphics::new()));
        Rustboy {
            cpu: CPU::new(graphics.clone()),
            graphics,
        }
    }

    pub fn run(&mut self) {
        self.cpu.boot();
        loop {
            self.cpu.step();
            self.graphics.borrow_mut().update();
        }
    }
}
