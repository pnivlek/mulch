use crate::constants;
use sdl2::render::Canvas;
use sdl2::video::Window;
use sdl2::Sdl;

pub fn init_graphics() -> (Sdl, Canvas<Window>) {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let window = video_subsystem
        .window("mulch - the chip-8 emulator", 64, 32)
        .position_centered()
        .build()
        .unwrap();

    (sdl_context, window.into_canvas().build().unwrap())
}

pub fn draw_graphics(canvas: &Canvas<Window>, gfx: &[u8; constants::GFX_LENGTH]) -> () {
    canvas.clear();
    for x in 0..constants::GFX_COLUMNS {
        for y in 0..constants::GFX_ROWS {
            if gfx[x * constants::GFX_ROWS + y * constants::GFX_COLUMNS] != 0 {
                canvas.draw_point(sdl2::rect::Point::new(x, y));
            }
        }
    }
    canvas.present();
}
