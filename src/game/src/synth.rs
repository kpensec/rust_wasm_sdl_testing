extern crate rand;
// used in noise gen 
// use self::rand::{Rng, thread_rng};

use sdl2::audio::{AudioStatus, AudioDevice, AudioCallback};
use utils::clamp;
use key::Key;
use periodical_wave::Oscilator_Type;


pub struct Synthesizer {
    pub volume: f32,
    pub freq: i32,
    sample_number: i32,
    keys: [Key; 25],
    // rng: Rng,
}

//pub fn get_white_noise_sample(noise: Noise, volume: f32, rng_number: f32) -> f32 {
//    noise.get_sample(rng_number) * volume
//}
//pub fn get_white_noise_sample(tone: i32, time: f32, volume: f32) -> f32 {
//    (rng.next_f32() as f32 * 2.0 - 1.0) * volume
//}


impl Synthesizer {
    pub fn change_volume(&mut self, q: f32) {
        self.volume = clamp(self.volume + q, 0.0, 1.0);
    }

    pub fn new(freq: i32) -> Self{
        Synthesizer {
            volume: 0.5,
            freq: freq,
            sample_number: 0,
            keys: [
                Key::new(262, Oscilator_Type::SINE),
                Key::new(294, Oscilator_Type::SINE),
                Key::new(330, Oscilator_Type::SINE),
                Key::new(349, Oscilator_Type::SINE),
                Key::new(392, Oscilator_Type::SINE),
                Key::new(440, Oscilator_Type::SINE),
                Key::new(494, Oscilator_Type::SINE),
                Key::new(523, Oscilator_Type::SINE),
                Key::new(262, Oscilator_Type::SAW),
                Key::new(294, Oscilator_Type::SAW),
                Key::new(330, Oscilator_Type::SAW),
                Key::new(349, Oscilator_Type::SAW),
                Key::new(392, Oscilator_Type::SAW),
                Key::new(440, Oscilator_Type::SAW),
                Key::new(494, Oscilator_Type::SAW),
                Key::new(523, Oscilator_Type::SAW),
                Key::new(262, Oscilator_Type::SQUARE),
                Key::new(294, Oscilator_Type::SQUARE),
                Key::new(330, Oscilator_Type::SQUARE),
                Key::new(349, Oscilator_Type::SQUARE),
                Key::new(392, Oscilator_Type::SQUARE),
                Key::new(440, Oscilator_Type::SQUARE),
                Key::new(494, Oscilator_Type::SQUARE),
                Key::new(523, Oscilator_Type::SQUARE),
                Key::new(100, Oscilator_Type::SQUARE),
                ],
        }
    }

    pub fn start_note(&mut self, key_idx: usize) {
        self.keys[key_idx].press()
    }

    pub fn release_note(&mut self, key_idx: usize) {
        self.keys[key_idx].release()
    }
}

fn mix_samples(lhs: f32, rhs: f32) -> f32 {
    if lhs != 0.0 && rhs != 0.0 { lhs * rhs } else { rhs + lhs }// - if lhs.signum() == rhs.signum() { lhs * rhs.abs() } else { 0.0 }
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


pub fn play<CB : AudioCallback>(device : &mut AudioDevice<CB>) {
    if device.status() == AudioStatus::Paused {
        device.resume();
    } else {
        device.pause();
    }
}
