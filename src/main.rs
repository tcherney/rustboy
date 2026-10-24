//! Start with https://rylev.github.io/DMG-01/public/book/ as reference
//! https://gbdev.io/pandocs/Specifications.html
//! first reference exhausted, have to go through the specifications
//! and imlement the rest of the CPU instructions
//! get raylib up and start testing instruction execution

use raylib;
mod cpu;
mod graphics;
mod memorybus;
mod registers;
mod rustboy;
use crate::rustboy::Rustboy;
fn main() {
    let mut rustboy = Rustboy::new();
    rustboy.run();
}
