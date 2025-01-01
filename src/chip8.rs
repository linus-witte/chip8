const RAM_SIZE: usize = 4096;
const STACK_SIZE: usize = 16;
const NUM_V_REGISTERS: usize = 16;
const GFX_SIZE: usize = 64 * 32;

const CHIP8_FONTSET: [u8; 80] = [
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

    delay_timer: u8,

    /// only used in 0xFX0A
    key_pressed: bool,

    pub key: [bool; 16],

    /// graphics buffer
    pub gfx: [bool; GFX_SIZE],
}

impl Chip {
    pub fn new() -> Self {
        let mut chip = Self {
            ram: [0; RAM_SIZE],
            stack: [0; STACK_SIZE],
            v: [0; NUM_V_REGISTERS],
            pc: 0x200,
            sp: 0,
            i: 0,
            delay_timer: 0,
            key: [false; 16],
            key_pressed: false,
            gfx: [false; GFX_SIZE],
        };
        for i in 0..80 {
            chip.ram[i] = CHIP8_FONTSET[i];
        }
        chip
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

        let x = ((opcode & 0x0F00) >> 8) as usize;
        let y = ((opcode & 0x00F0) >> 4) as usize;
        let nn = (opcode & 0x00FF) as u8;

        match opcode & 0xF000 {
            0x0000 => match opcode & 0x0FFF {
                0x0E0 => self.gfx = [false; GFX_SIZE],
                0x0EE => {
                    self.sp -= 1;
                    self.pc = self.stack[self.sp as usize];
                }
                _ => panic!("unrecognized opcode: {:04x}", opcode),
            },

            // 0x1NNN: Sets the program counter to address NNN (jump instruction).
            0x1000 => self.pc = opcode & 0x0FFF,

            // 0x2NNN: Calls subroutine at NNN
            0x2000 => {
                self.stack[self.sp as usize] = self.pc;
                self.sp += 1;
                self.pc = opcode & 0x0FFF
            }

            // 0x3XNN: Skips the next instruction if VX equals NN.
            0x3000 => {
                if self.v[x] == nn {
                    self.pc += 2
                }
            }

            // 0x4XNN: Skips the next instruction if VX does not equal NN.
            0x4000 => {
                if self.v[x] != nn {
                    self.pc += 2
                }
            }

            // 0x5XNN: Skips the next instruction if VX equals VY .
            0x5000 => {
                if self.v[x] == self.v[y] {
                    self.pc += 2
                }
            }

            // 0x6XNN: Sets register VX to the value NN.
            0x6000 => self.v[x] = nn,

            // 0x7XNN: Adds NN to VX.
            0x7000 => self.v[x] = self.v[x].wrapping_add(nn),

            // 0x8XYN: bit and math operations
            0x8000 => match opcode & 0x000F {
                0x0 => self.v[x] = self.v[y],
                0x1 => {
                    self.v[x] |= self.v[y];
                    self.v[0xF] = 0
                }
                0x2 => {
                    self.v[x] &= self.v[y];
                    self.v[0xF] = 0
                }
                0x3 => {
                    self.v[x] ^= self.v[y];
                    self.v[0xF] = 0
                }
                0x4 => {
                    let cf = if (self.v[x] as u16) + (self.v[y] as u16) > 0xFF {
                        1
                    } else {
                        0
                    };
                    self.v[x] = self.v[x].wrapping_add(self.v[y]);
                    self.v[0xF] = cf;
                }
                0x5 => {
                    let cf = if self.v[y] > self.v[x] { 0 } else { 1 };
                    self.v[x] = self.v[x].wrapping_sub(self.v[y]);
                    self.v[0xF] = cf;
                }
                0x6 => {
                    let lsb = self.v[y] & 0x1;
                    self.v[x] = self.v[y] >> 1;
                    self.v[0xF] = lsb;
                }
                0x7 => {
                    let cf = if self.v[x] > self.v[y] { 0 } else { 1 };
                    self.v[x] = self.v[y].wrapping_sub(self.v[x]);
                    self.v[0xF] = cf;
                }
                0xE => {
                    let msb = (self.v[y] & 0x80) >> 7;
                    self.v[x] = self.v[y] << 1;
                    self.v[0xF] = msb;
                }
                _ => panic!("unrecognized opcode: {:04x}", opcode),
            },

            // 0x9XYN: Skips the next instruction if VX does not equal VY.
            0x9000 => {
                if self.v[x] != self.v[y] {
                    self.pc += 2
                }
            }

            // 0xANNN: Sets the index register (I) to address NNN.
            0xA000 => self.i = opcode & 0x0FFF,

            // 0xBNNN: Jumps to the address NNN plus V0.
            0xB000 => self.pc = (opcode & 0x0FFF) + self.v[0] as u16,

            // 0xCXNN: Sets VX to the result of a bitwise and operation on a random number and NN.
            0xC000 => self.v[x] = rand::random::<u8>() & nn,

            // 0xDXYN: Draws a sprite at coordinates (VX, VY) with height N pixels.
            0xD000 => self.draw(&opcode),

            0xE000 => match opcode & 0x00FF {
                // 0xEX9E: Skips the next instruction if the key stored in VX is pressed
                0x009E => {
                    if self.key[self.v[x] as usize] {
                        self.pc += 2;
                    }
                }
                // 0xEXA1: Skips the next instruction if the key stored in VX is not pressed
                0x00A1 => {
                    if !self.key[self.v[x] as usize] {
                        self.pc += 2;
                    }
                }
                _ => panic!("unrecognized opcode: {:04x}", opcode),
            },

            0xF000 => match opcode & 0x00FF {
                0x0007 => self.v[x] = self.delay_timer,

                0x0015 => self.delay_timer = self.v[x],

                // 0xFX0A: A key press is awaited, and then stored in VX (blocking operation)
                0x000A => {
                    if self.key_pressed {
                        // wait for release
                        if self.key[self.v[x] as usize] {
                            self.pc -= 2;
                        } else {
                            self.key_pressed = false;
                        }
                    } else {
                        for i in 0..16 {
                            if self.key[i] {
                                self.v[x] = i as u8;
                                self.key_pressed = true;
                                break;
                            }
                        }
                        self.pc -= 2;
                    }
                }

                // 0xFX1E: Stores the binary-coded decimal representation of VX, with the hundreds digit in memory at location in I, the tens digit at location I+1, and the ones digit at location I+2
                0x001E => {
                    self.i = self.i.wrapping_add(self.v[x] as u16);
                }
                // 0xFX33: Stores the binary-coded decimal representation of VX, with the hundreds digit in memory at location in I, the tens digit at location I+1, and the ones digit at location I+2
                0x0033 => {
                    self.ram[self.i as usize] = self.v[x] / 100;
                    self.ram[self.i as usize + 1] = (self.v[x] % 100) / 10;
                    self.ram[self.i as usize + 2] = self.v[x] % 10;
                }
                // 0xFX55: Stores from V0 to VX (including VX) in memory, starting at address I.
                0x0055 => {
                    for i in 0..=x {
                        self.ram[self.i as usize + i] = self.v[i];
                    }
                    self.i = self.i + x as u16 + 1;
                }
                // 0xFX65: Fills from V0 to VX (including VX) with values from memory, starting at address I.
                0x0065 => {
                    for i in 0..=x {
                        self.v[i] = self.ram[self.i as usize + i];
                    }
                    self.i = self.i + x as u16 + 1;
                }
                _ => panic!("unrecognized opcode: {:04x}", opcode),
            },
            _ => panic!("unrecognized opcode: {:04x}", opcode),
        }

        if self.delay_timer > 0 {
            self.delay_timer = self.delay_timer - 1;
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

    pub fn draw_flag(&self) -> bool {
        self.v[0xF] == 1
    }
}
