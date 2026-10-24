use crate::raylib::RaylibHandle;
use crate::raylib::RaylibThread;
use raylib::prelude::*;

pub(crate) const VRAM_BEGIN: usize = 0x8000;
pub(crate) const VRAM_END: usize = 0x9FFF;
pub(crate) const VRAM_SIZE: usize = VRAM_END - VRAM_BEGIN + 1;

#[derive(Copy, Clone)]
enum TilePixelValue {
    Zero,
    One,
    Two,
    Three,
}

type Tile = [[TilePixelValue; 8]; 8];
fn empty_tile() -> Tile {
    [[TilePixelValue::Zero; 8]; 8]
}
pub struct Graphics {
    vram: [u8; VRAM_SIZE],
    tiles: [Tile; 384],
    window: RaylibHandle,
    thread: RaylibThread,
}

impl Graphics {
    pub fn new() -> Self {
        let (mut window, thread) = raylib::init().size(160, 144).title("Rustboy").build();
        window.set_target_fps(60);
        Graphics {
            vram: [0; VRAM_SIZE],
            tiles: [empty_tile(); 384],
            window,
            thread,
        }
    }

    pub fn update(&mut self) {
        let mut d = self.window.begin_drawing(&self.thread);
        d.clear_background(Color::WHITE);
        for tile_index in 0..self.tiles.len() {
            let tile = &self.tiles[tile_index];
            for y in 0..8 {
                for x in 0..8 {
                    let pixel_value = tile[y][x];
                    let color = match pixel_value {
                        TilePixelValue::Zero => Color::WHITE,
                        TilePixelValue::One => Color::LIGHTGRAY,
                        TilePixelValue::Two => Color::DARKGRAY,
                        TilePixelValue::Three => Color::BLACK,
                    };
                    d.draw_pixel(
                        x as i32 + (tile_index % 16) as i32 * 8,
                        y as i32 + (tile_index / 16) as i32 * 8,
                        color,
                    );
                }
            }
        }
    }
    pub fn write_vram(&mut self, address: u16, value: u8) {
        let index = (address - VRAM_BEGIN as u16) as usize;
        if index < VRAM_SIZE {
            self.vram[index] = value;
            if index >= 0x1800 {
                return;
            }
            let normalized_index = index & 0xFFFE;
            let byte1 = self.vram[normalized_index];
            let byte2 = self.vram[normalized_index + 1];
            let tile_index = index / 16;
            let row_index = (index % 16) / 2;
            for pixel_index in 0..8 {
                let bit1 = (byte1 >> (7 - pixel_index)) & 1;
                let bit2 = (byte2 >> (7 - pixel_index)) & 1;
                let pixel_value = match (bit1, bit2) {
                    (0, 0) => TilePixelValue::Zero,
                    (0, 1) => TilePixelValue::One,
                    (1, 0) => TilePixelValue::Two,
                    (1, 1) => TilePixelValue::Three,
                    _ => unreachable!(),
                };
                self.tiles[tile_index][row_index][pixel_index] = pixel_value;
            }
        }
    }
    pub fn read_vram(&self, address: u16) -> u8 {
        let index = (address - VRAM_BEGIN as u16) as usize;
        if index < VRAM_SIZE {
            self.vram[index]
        } else {
            0
        }
    }
}
