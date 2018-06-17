extern crate rand;
// used in noise gen
// use self::rand::{Rng, thread_rng};

use sdl2::audio::{AudioStatus, AudioDevice, AudioCallback};

use utils::clamp;

use synth::key::Key;
use synth::periodical_wave::OscilatorType;
use synth::envelop::Envelop;


pub struct Synthesizer {
    pub volume: f32,
    pub freq: i32,
    pub envelop: Envelop,
    sample_number: i32,
    keys: [Key; 32],
    // rng: Rng,
}

impl Synthesizer {
    pub fn change_volume(&mut self, q: f32) {
        self.volume = clamp(self.volume + q, 0.0, 1.0);
    }

    pub fn new(freq: i32) -> Self{
        Synthesizer {
            volume: 0.2,
            freq: freq,
            envelop: Envelop{
                attack: 0.0,
                decay: 0.5,
                release: 1.0,
                peakAmp: 1.0,
                sustainAmp: 0.5,

            },
            sample_number: 0,
            keys: [
                Key::new(0, OscilatorType::Sine),
                Key::new(1, OscilatorType::Sine),
                Key::new(2, OscilatorType::Sine),
                Key::new(3, OscilatorType::Sine),
                Key::new(4, OscilatorType::Sine),
                Key::new(5, OscilatorType::Sine),
                Key::new(6, OscilatorType::Sine),
                Key::new(7, OscilatorType::Sine),
                Key::new(8, OscilatorType::Sine),
                Key::new(9, OscilatorType::Sine),
                Key::new(10, OscilatorType::Sine),
                Key::new(11, OscilatorType::Sine),
                Key::new(12, OscilatorType::Sine),
                Key::new(7, OscilatorType::Saw),
                Key::new(7, OscilatorType::Saw),
                Key::new(7, OscilatorType::Saw),
                Key::new(7, OscilatorType::Square),
                Key::new(7, OscilatorType::Square),
                Key::new(7, OscilatorType::Square),
                Key::new(7, OscilatorType::Square),
                Key::new(7, OscilatorType::Square),
                Key::new(7, OscilatorType::Square),
                Key::new(7, OscilatorType::Square),
                Key::new(7, OscilatorType::Square),
                Key::new(7, OscilatorType::Triangle),
                Key::new(7, OscilatorType::Triangle),
                Key::new(7, OscilatorType::Triangle),
                Key::new(7, OscilatorType::Triangle),
                Key::new(7, OscilatorType::Triangle),
                Key::new(7, OscilatorType::Triangle),
                Key::new(7, OscilatorType::Triangle),
                Key::new(7, OscilatorType::Triangle),
                ],
        }
    }

    pub fn start_note(&mut self, key_idx: usize) {
        self.keys[key_idx].press()
    }

    pub fn release_note(&mut self, key_idx: usize) {
        self.keys[key_idx].release()
    }

    pub fn toggle_audio(device: &mut AudioDevice<Self>) {
        if device.status() == AudioStatus::Paused {
            device.resume();
        } else {
            device.pause();
        }
    }
}

fn mix_samples(lhs: f32, rhs: f32) -> f32 {
    lhs + rhs
    //if lhs != 0.0 && rhs != 0.0 { lhs * rhs } else { rhs + lhs }// - if lhs.signum() == rhs.signum() { lhs * rhs.abs() } else { 0.0 }
}

impl AudioCallback for Synthesizer {
    type Channel = f32;


    fn callback(&mut self, out: &mut [f32]) {
        // Generate a square wave
        let mut current: f32 = 0.0;
        let mut first = true;
        // let ifreq: f32 = 1.0 / self.freq as f32;
        for x in out.iter_mut() {
            if first {
                current = 0.0;
                for key in self.keys.iter_mut() {
                    if key.active {
                        current = mix_samples(current, key.update(self.freq, self.volume));
                    }
                }
                self.sample_number += 1;
            }
            first = ! first;
            *x = current;
        }
    }

}

