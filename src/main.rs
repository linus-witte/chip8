mod chip8;

fn main() {
    let chip = chip8::Chip::new();
    println!("{:#?}", chip);
}
