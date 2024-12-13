const RAM_SIZE: usize = 4096;
const STACK_SIZE: usize = 16;
const NUM_V_REGISTERS: usize = 16;

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
        }
    }
}
