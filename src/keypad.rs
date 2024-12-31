use sdl2::{event::Event, keyboard::Keycode, EventPump};

fn keymap(key: Keycode) -> Option<u8> {
    match key {
        Keycode::Num1 => Some(0x1),
        Keycode::Num2 => Some(0x2),
        Keycode::Num3 => Some(0x3),
        Keycode::Num4 => Some(0xc),
        Keycode::Q => Some(0x4),
        Keycode::W => Some(0x5),
        Keycode::F => Some(0x6),
        Keycode::P => Some(0xd),
        Keycode::A => Some(0x7),
        Keycode::R => Some(0x8),
        Keycode::S => Some(0x9),
        Keycode::T => Some(0xe),
        Keycode::X => Some(0xa),
        Keycode::C => Some(0x0),
        Keycode::D => Some(0xb),
        Keycode::V => Some(0xf),
        _ => None,
    }
}

pub struct Keypad {
    event_pump: EventPump,
}

impl Keypad {
    pub fn new(sdl_context: &sdl2::Sdl) -> Result<Self, String> {
        sdl_context
            .event_pump()
            .map(|event_pump| Keypad { event_pump })
    }

    pub fn poll(&mut self) -> Result<[bool; 16], ()> {
        for event in self.event_pump.poll_iter() {
            if let Event::Quit { .. } = event {
                return Err(());
            }
        }

        let mut keys = [false; 16];

        self.event_pump
            .keyboard_state()
            .pressed_scancodes()
            .filter_map(Keycode::from_scancode)
            .filter_map(keymap)
            .for_each(|i| keys[i as usize] = true);

        Ok(keys)
    }
}
