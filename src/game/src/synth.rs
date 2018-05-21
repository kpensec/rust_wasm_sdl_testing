use sdl2::audio::{AudioStatus, AudioDevice, AudioCallback};
use std::f32::consts::PI;
use utils::clamp;

pub struct SquareWave {
    pub volume: f32,
    pub freq: i32,
    pub tones: [i32; 4],
    pub phases: [f32; 4],
    sample_number: i32,
    keys: [Key; 8]
}

#[derive(Clone, Copy)]
struct Key {
    pub active: bool,
    pub down: bool,
    pub attack_duration: f32,
    pub release_duration: f32,
    pub tone: i32,
    pub volume: f32,

    attack_end: f32,
    release_end: f32,
    time: f32,
}

impl Key {
    pub fn new(tone: i32) -> Self {
        Key {
            active: false,
            down: false,
            attack_duration: 1.0,
            release_duration: 1.5,
            tone: tone,
            volume: 0.0,

            time: 2.0,
            attack_end: 0.0,
            release_end: 0.0,
        }
    }
    pub fn press(&mut self) {
        if ! self.down { // ignore key repeat
            self.down = true;
            self.attack_end = self.attack_duration;
        }
    }

    pub fn release(&mut self) {
        self.down = false;
        self.release_end = self.time + self.release_duration * self.volume
        self.release_end = self.time;
    }

    pub fn update(&mut self, freq: i32, volume: f32) -> f32 {
        if self.active {
            if self.down {
                self.time = self.time + 1.0 / freq as f32;
                self.volume = (self.time / self.attack_end).min(1.0);
            } else {
                self.time = (self.time + 1.0 / freq as f32).min(self.release_end);
                self.volume = ((self.release_end - self.time) / self.release_end).max(0.0);
                self.active = volume == 0.0;
            }
            get_sample(self.tone, self.time, volume * self.volume)
        } else {
            0.0
        }
    }
}

pub fn get_sample(tone: i32, time: f32, volume: f32) -> f32 {
    clamp((PI * 2.0 * time * tone as f32).sin() * volume, -1.0, 1.0)
}

impl SquareWave {
    pub fn change_tone(&mut self, i: usize, q: i32) {
        self.tones[i] = clamp(self.tones[i] + q, 110, 1760);
    }

    pub fn change_volume(&mut self, q: f32) {
        self.volume = clamp(self.volume + q, 0.0, 1.0);
    }
    pub fn change_phase(&mut self, i: usize, q: f32) {
        self.phases[i] += q;
        println!("new phase -> {}",self.phases[i]);
    }

    pub fn new(freq: i32) -> Self{
        // const TONES : [i32: 8] = [
        //     262, 294, 330, 349,
        //     392, 440, 494, 523
        // ];
        let keys = [
            Key::new(262),
            Key::new(294),
            Key::new(330),
            Key::new(349),
            Key::new(392),
            Key::new(440),
            Key::new(494),
            Key::new(523),
        ];
        SquareWave {
            volume: 0.5,
            freq: freq,
            tones: [440; 4],
            phases: [0.0; 4],
            sample_number: 0,
            keys: keys
        }
    }

    pub fn get_current_sample(&self, tone: i32, phase: f32) -> f32 {
        get_sample(tone, (phase + self.sample_number as f32) / self.freq as f32, self.volume)
    }

    pub fn start_note(&mut self, key_idx: usize) {
        self.keys[key_idx].press()
    }

    pub fn release_note(&mut self, key_idx: usize) {
        self.keys[key_idx].release()
    }
}

fn mix_samples(lhs: f32, rhs: f32) -> f32 {
    lhs + rhs - if lhs.signum() == rhs.signum() { lhs * rhs.abs() } else { 0.0 }
}

impl AudioCallback for SquareWave {
    type Channel = f32;


    fn callback(&mut self, out: &mut [f32]) {
        // Generate a square wave
        let mut current: f32 = 0.0;
        let mut first = true;
        let ifreq: f32 = 1.0 / self.freq as f32;
        for x in out.iter_mut() {
            if first {
                current = 0.0;
                //for i in 0..1 {
                //    let new_sample = self.get_current_sample(self.tones[i], self.phases[i] * ifreq);
                //    current = mix_samples(current, new_sample)
                //}
                for key in self.keys.iter_mut() {
                    current = mix_samples(current, key.update(self.freq, self.volume));
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
