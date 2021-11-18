use std::convert::TryInto;

use crate::constants;
use sdl2::render::Canvas;
use sdl2::video::Window;
use sdl2::Sdl;

pub struct Graphics {
    sdl_context: Sdl,
    canvas: Canvas<Window>,
}

impl Graphics {
    pub fn new() -> Graphics {
        let sdl_context = sdl2::init().unwrap();
        let video_subsystem = sdl_context.video().unwrap();

        let window = video_subsystem
            .window("mulch - the chip-8 emulator", 64, 32)
            .position_centered()
            .build()
            .unwrap();

        let graphics = Graphics {
            sdl_context,
            canvas: window.into_canvas().build().unwrap(),
        };

        graphics
    }

    pub fn draw_graphics(&mut self, gfx: &[u8; constants::GFX_LENGTH]) -> () {
        self.canvas.clear();
        for x in 0..constants::GFX_COLUMNS {
            for y in 0..constants::GFX_ROWS {
                if gfx[x * constants::GFX_ROWS + y * constants::GFX_COLUMNS] != 0 {
                    self.canvas.draw_point(sdl2::rect::Point::new(
                        x.try_into().unwrap(),
                        y.try_into().unwrap(),
                    ));
                }
            }
        }
        self.canvas.present();
    }
}
