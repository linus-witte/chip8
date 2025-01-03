use sdl2;
use sdl2::audio::{AudioCallback, AudioDevice, AudioSpecDesired};
use std::f32::consts::PI;

const SAMPLE_RATE: i32 = 44100;
const FREQUENCY: f32 = 440.0; // A4

struct AudioSpec {
    phase_inc: f32,
    phase: f32,
    volume: f32,
}

pub struct AudioDriver {
    device: AudioDevice<AudioSpec>,
}

impl AudioDriver {
    pub fn new(sdl_context: &sdl2::Sdl) -> Result<AudioDriver, String> {
        let audio_subsystem = sdl_context.audio().unwrap();

        let desired_spec = AudioSpecDesired {
            freq: Some(SAMPLE_RATE),
            channels: Some(1),
            samples: None,
        };

        audio_subsystem
            .open_playback(None, &desired_spec, |spec| AudioSpec {
                phase_inc: FREQUENCY as f32 / spec.freq as f32,
                phase: 0.0,
                volume: 0.5,
            })
            .map(|device| AudioDriver { device })
    }

    pub fn start(&self) {
        self.device.resume();
    }
    pub fn stop(&self) {
        self.device.pause();
    }
}

impl AudioCallback for AudioSpec {
    type Channel = f32;

    fn callback(&mut self, out: &mut [f32]) {
        // Generate a sine wave
        for x in out.iter_mut() {
            *x = self.volume * (2.0 * PI * self.phase).sin();
            self.phase = (self.phase + self.phase_inc) % 1.0;
        }
    }
}
