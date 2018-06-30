
mod key;
mod periodical_wave;
mod noise;
mod envelop;
mod note;
mod instrument;

use utils::clamp;
use synth::key::Key;
use std::vec::Vec;
use sdl2::audio::AudioCallback;

pub struct Synthesizer {
    volume: f32,
    buffer_size: usize,
    keys: [Key; 13],
    step: f32,
    active: bool,
}

impl Synthesizer {

    pub fn new(playback_freq: i32) -> Self{
        Synthesizer {
            volume: 1.0,
            buffer_size: 2048,
            keys: [ Key::new(0), Key::new(1), Key::new(2), Key::new(3), Key::new(4), Key::new(5), Key::new(6),
                Key::new(7), Key::new(8), Key::new(9), Key::new(10), Key::new(11), Key::new(12), ],
            step: 1.0 / playback_freq as f32,
            active: true
        }
    }
    pub fn is_active(&self) -> bool {
        self.active
    }

    pub fn get_volume(&self) -> f32 {
        self.volume
    }
    pub fn set_volume(&mut self, q: f32) {
        self.volume = clamp(self.volume + q, 0.0, 1.0);
    }

    pub fn start_note(&mut self, key_idx: usize) {
        // TODO vec upsert here!
        self.keys[key_idx].press()
    }

    pub fn release_note(&mut self, key_idx: usize) {
        // TODO vec update here!
        self.keys[key_idx].release()
    }

    fn blend_sample(lhs: f32, rhs: f32) -> f32 {
        // TODO read a book about sample blending/polyphony!
        lhs + rhs
    }

    fn get_sample(&mut self) -> f32 {
        let mut result = 0.0;
        for key in self.keys.iter_mut() {
            result = Self::blend_sample(result, key.update(self.volume, self.step));
        }
        result
    }

    pub fn update(&mut self, _eps: f32) -> Vec<f32> {
        let mut result = Vec::<f32>::with_capacity(self.buffer_size);
        if ! self.active {
            ()
        }

        for _ in 0usize..self.buffer_size {
            let sample = self.get_sample();
            result.push(sample);
            result.push(sample);
        }
        result
    }

    pub fn toggle_audio(&mut self) -> (){
        self.active = ! self.active;
    }

}

impl AudioCallback for Synthesizer {
    type Channel = f32;

    fn callback(&mut self, output: &mut [f32]) {
        let mut chan = 0;
        if self.is_active() {
            let mut sample = 0.0;
            for x in output.iter_mut() {
                if chan == 0 {
                    sample = self.get_sample();
                }
                chan = chan + 1;
                if chan > 2 {
                    chan = 0
                }
                *x = sample;
            }
        } else {
            // nullify buffer
            for x in output.iter_mut() {
                *x = 0.0;
            }
        }
    }
}

