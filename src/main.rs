mod chip8;
use chip8::Chip;
mod display;
use display::Display;

use std::fs::File;
use std::io::Read;

fn read_rom(path: &str) -> Result<Vec<u8>, std::io::Error> {
    let mut file = File::open(path)?;
    let mut buffer = Vec::new();

    file.read_to_end(&mut buffer)?;
    Ok(buffer)
}

fn main() -> Result<(), String> {
    let args: Vec<String> = std::env::args().collect();

    let sdl_context = sdl2::init().unwrap();
    let mut display = Display::init(&sdl_context)?;

    ctrlc::set_handler(move || {
        println!("Received termination signal. Shutting down gracefully...");
        std::process::exit(0);
    })
    .expect("Error setting signal handler");

    let mut chip = Chip::new();
    let rom = read_rom(&args[1]).map_err(|e| e.to_string())?;

    chip.load_rom(rom);

    loop {
        chip.emulate_cycle();

        if chip.draw_flag() {
            display.draw(&chip.gfx);
        }

        std::thread::sleep(std::time::Duration::from_millis(2));
    }
}
