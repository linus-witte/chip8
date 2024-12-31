use sdl2::{pixels::Color, rect::Rect, render::Canvas, video::Window};

const WIDTH: usize = 64;
const HEIGHT: usize = 32;

const SCALE: u32 = 16;
const SCREEN_WIDTH: u32 = (WIDTH as u32) * SCALE;
const SCREEN_HEIGHT: u32 = (HEIGHT as u32) * SCALE;

pub struct Display {
    canvas: Canvas<Window>,
}

impl Display {
    pub fn init(sdl_context: &sdl2::Sdl) -> Result<Display, String> {
        let video_subsystem = sdl_context.video()?;
        let window = video_subsystem
            .window("Chip8", SCREEN_WIDTH, SCREEN_HEIGHT)
            .opengl()
            .build()
            .map_err(|e| e.to_string())?;

        let canvas = window.into_canvas().build().map_err(|e| e.to_string())?;

        Ok(Display { canvas })
    }

    pub fn draw(&mut self, pixels: &[bool; WIDTH * HEIGHT]) {
        self.canvas.clear();
        for y in 0..HEIGHT {
            for x in 0..WIDTH {
                let idx = (y * WIDTH) + x as usize;
                self.canvas.set_draw_color(if pixels[idx] {
                    Color::WHITE
                } else {
                    Color::BLACK
                });
                let _ = self.canvas.fill_rect(Rect::new(
                    (x as i32) * SCALE as i32,
                    (y as i32) * SCALE as i32,
                    SCALE,
                    SCALE,
                ));
            }
        }
        self.canvas.present();
    }
}
