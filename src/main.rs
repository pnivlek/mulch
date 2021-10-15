mod chip8;
mod constants;
mod graphics;
extern crate sdl2;

use chip8::Chip8;
use sdl2::render::Canvas;
use sdl2::video::Window;
use sdl2::Sdl;

fn main() {
    let (sdl_context, canvas): (Sdl, Canvas<Window>) = graphics::init_graphics();

    // Initialize chip8 machine
    let mut chip8 = Chip8::new();
    // Load the ROM
    chip8.load_rom("./rom/pong.rom").expect("Loading ROM:");

    loop {
        chip8.emulate_cycle();

        if chip8.draw_flag != 0 {
            graphics::draw_graphics(&canvas, chip8.gfx());
            chip8.draw_flag = 0;
        }
    }
}
