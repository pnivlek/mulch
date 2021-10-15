mod utils;
mod constants;

fn draw_graphics(gfx: &[u8; constants::GFX_LENGTH]) -> () {

}

fn main() {
    // Register input callbacks
    // registerCallbacks();

    // Initialize chip8 machine
    let mut machine: utils::Chip8 = Default::default();
    // Load the ROM
    machine.load_rom("./rom/pong.rom").expect("Loading ROM:");

    loop {
        machine.emulate_cycle();

        if machine.draw_flag != 0 {
            draw_graphics(machine.gfx());
            machine.draw_flag = 0;
        }

        machine.set_keys();
    }

}
