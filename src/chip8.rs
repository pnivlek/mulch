use crate::constants;
use std::fs::File;
use std::io::{self, Error, ErrorKind, Read};
use std::convert::TryInto;

pub struct Chip8 {
    opcode: u16,
    memory: [u8; constants::MEMORY_SIZE],
    v: [u8; 16], // registers V0-VE, with carry flag in 16th register

    i_reg: u16,
    pc_reg: u16,

    gfx: [u8; constants::GFX_LENGTH],

    delay_timer: u8,
    sound_timer: u8,

    stack: [u16; 16],
    sp: u16,

    keypad: [u8; 16],

    pub draw_flag: u8,
}

impl Chip8 {
    pub fn new() -> Chip8 {
        Chip8 {
            opcode: 0,
            memory: [0; 4096],
            v: [0; 16],

            i_reg: 0,
            pc_reg: 0,

            gfx: [0; constants::GFX_LENGTH],

            delay_timer: 0,
            sound_timer: 0,

            stack: [0; 16],
            sp: 0,

            keypad: [0; 16],

            draw_flag: 0,
        }
    }
}

impl Chip8 {
    pub fn load_rom(&mut self, file_name: &str) -> io::Result<()> {
        let mut f = File::open(file_name)?;

        // Check to make sure the file isn't too large, to avoid the obvious buffer overflow.
        if f.metadata()?.len() > (constants::MEMORY_SIZE).try_into().unwrap() {
            // Switch to ErrorKind::FileTooLarge when it comes out of nightly API.
            return Err(Error::new(
                ErrorKind::InvalidInput,
                "ROM file cannot be larger than the memory of CHIP 8.",
            ));
        }
        f.read(&mut self.memory[0x200..])?;
        Ok(())
    }

    pub fn gfx(&self) -> &[u8; constants::GFX_LENGTH] {
        &self.gfx
    }

    pub fn set_keys(&mut self) -> () {}

    pub fn emulate_cycle(&mut self) -> () {

    }
}
