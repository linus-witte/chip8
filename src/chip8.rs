const RAM_SIZE: usize = 4096;
const STACK_SIZE: usize = 16;
const NUM_V_REGISTERS: usize = 16;
const GFX_SIZE: usize = 64 * 32;

#[derive(Debug)]
pub struct Chip {
    ram: [u8; RAM_SIZE],
    stack: [u16; STACK_SIZE],

    /// general purpose registers V0 to VE. VF is the carry flag.
    v: [u8; NUM_V_REGISTERS],
    /// program counter
    pc: u16,
    /// stack pointer
    sp: u16,
    /// index register
    i: u16,

    /// graphics buffer
    pub gfx: [bool; GFX_SIZE],
}

impl Chip {
    pub fn new() -> Self {
        Self {
            ram: [0; RAM_SIZE],
            stack: [0; STACK_SIZE],
            v: [0; NUM_V_REGISTERS],
            pc: 0x200,
            sp: 0,
            i: 0,
            gfx: [false; GFX_SIZE],
        }
    }

    /// Loads the provided rom data into memory. Starting at 0x200;
    pub fn load_rom(&mut self, rom: Vec<u8>) {
        let end = 0x200 + Vec::len(&rom);
        self.ram[0x200..end].copy_from_slice(&rom);
    }

    /// Fetches the current opcode from memory at the program counter and increments the program counter by 2.
    fn fetch_opcode(&mut self) -> u16 {
        let opcode =
            ((self.ram[self.pc as usize] as u16) << 8) | (self.ram[self.pc as usize + 1] as u16);
        self.pc += 2;
        opcode
    }

    /// Emulates a single cycle of the Chip-8 interpreter by fetching, decoding, and executing an opcode.
    pub fn emulate_cycle(&mut self) {
        let opcode = self.fetch_opcode();

        if opcode == 0x00E0 {
            self.gfx = [false; GFX_SIZE]; // Clear display
            return;
        }

        match opcode & 0xF000 {
            // 0x1NNN: Sets the program counter to address NNN (jump instruction).
            0x1000 => self.pc = opcode & 0x0FFF,

            // 0x6XNN: Sets register VX to the value NN.
            0x6000 => self.v[((opcode & 0x0F00) >> 8) as usize] = (opcode & 0x00FF) as u8,

            // 0xANNN: Sets the index register (I) to address NNN.
            0xA000 => self.i = opcode & 0x0FFF,

            // 0xDXYN: Draws a sprite at coordinates (VX, VY) with height N pixels.
            0xD000 => self.draw(&opcode),

            _ => println!("\tunrecognized opcode: {:04x}", opcode),
        }
    }

    fn draw(&mut self, opcode: &u16) {
        let x = self.v[((opcode & 0x0F00) >> 8) as usize] as usize;
        let y = self.v[((opcode & 0x00F0) >> 4) as usize] as usize;
        let height = (opcode & 0x000F) as usize;

        self.v[0xF] = 0;
        for y_offset in 0..height {
            let pixel = self.ram[self.i as usize + y_offset];
            let p_y = y_offset + y;
            if p_y >= 32 {
                break;
            }

            for x_offset in 0..8 {
                let p_x = x_offset + x;
                if p_x >= 64 {
                    break;
                }
                if (pixel & (0x80 >> x_offset)) != 0 {
                    self.gfx[p_x + (p_y * 64)] ^= true;
                    self.v[0xF] |= 1;
                }
            }
        }
    }
}
