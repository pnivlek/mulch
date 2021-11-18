use crate::constants;
use std::convert::TryInto;
use std::fs::File;
use std::io::{self, Error, ErrorKind, Read};

pub struct Chip8 {
    // Memory
    memory: [u8; constants::MEMORY_SIZE],

    // Registers: V0-VE, carry flag in 16th register.
    v: [u8; 16],

    // Index register I.
    i_reg: u16,

    // Program counter (PC)
    pc_reg: u16,

    // Graphics array.
    gfx: [u8; constants::GFX_LENGTH],

    // Delay timer
    delay_timer: u8,
    // Sound timer
    sound_timer: u8,

    // Stack of addresses
    stack: [u16; 16],
    // Stack pointer
    sp: u16,

    // Have we drawn to the screen?
    pub draw_flag: u8,
    // Have we cleared the screen?
    pub clear_flag :u8,
}

impl Chip8 {
    pub fn new() -> Chip8 {
        let mut chip8 = Chip8 {
            memory: [0; 4096],
            v: [0; 16],

            i_reg: 0,
            pc_reg: 0,

            gfx: [0; constants::GFX_LENGTH],

            delay_timer: 0,
            sound_timer: 0,

            stack: [0; constants::STACK_SIZE],
            sp: 0,

            draw_flag: 0,
            clear_flag: 0,
        };

        // Load fonts into system memory
        let fonts: [u8; 80] = [
            0xF0, 0x90, 0x90, 0x90, 0xF0, // 0
            0x20, 0x60, 0x20, 0x20, 0x70, // 1
            0xF0, 0x10, 0xF0, 0x80, 0xF0, // 2
            0xF0, 0x10, 0xF0, 0x10, 0xF0, // 3
            0x90, 0x90, 0xF0, 0x10, 0x10, // 4
            0xF0, 0x80, 0xF0, 0x10, 0xF0, // 5
            0xF0, 0x80, 0xF0, 0x90, 0xF0, // 6
            0xF0, 0x10, 0x20, 0x40, 0x40, // 7
            0xF0, 0x90, 0xF0, 0x90, 0xF0, // 8
            0xF0, 0x90, 0xF0, 0x10, 0xF0, // 9
            0xF0, 0x90, 0xF0, 0x90, 0x90, // A
            0xE0, 0x90, 0xE0, 0x90, 0xE0, // B
            0xF0, 0x80, 0x80, 0x80, 0xF0, // C
            0xE0, 0x90, 0x90, 0x90, 0xE0, // D
            0xF0, 0x80, 0xF0, 0x80, 0xF0, // E
            0xF0, 0x80, 0xF0, 0x80, 0x80, // F
        ];

        chip8.memory[..80].copy_from_slice(&fonts);

        return chip8;
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


    pub fn emulate_cycle(&mut self) -> () {
        // Read instruction.
        // Do instruction.
    }
}
