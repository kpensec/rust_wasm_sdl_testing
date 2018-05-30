use sdl2::audio::{AudioStatus, AudioDevice, AudioCallback};
use std::f32::consts::PI;
use utils::clamp;

extern crate rand;
use self::rand::{Rng, thread_rng};

// TODO remove/rework me?
pub struct SquareWave {
    pub volume: f32,
    pub freq: i32,
    pub tones: [i32; 4],
    pub phases: [f32; 4],
    sample_number: i32,
    keys: [Key; 25]
}

#[derive(Clone, Copy)]
pub enum Oscilator_Type {
    SINE,
    SAW,
    PULSE,
    WHITE_NOISE,
    SQUARE,
}

#[derive(Clone, Copy)]
struct Key {
    pub active: bool,
    pub down: bool,
    pub attack_duration: f32,
    pub release_duration: f32,
    pub tone: i32,
    pub volume: f32,
    osc_type: Oscilator_Type,
    attack_end: f32,
    release_end: f32,
    time: f32,
}


impl Key {
    pub fn new(tone: i32, osc_type: Oscilator_Type) -> Self {
        Key {
            active: false,
            down: false,
            attack_duration: 1.0,
            release_duration: 1.5,
            tone: tone,
            volume: 0.0,

            osc_type: osc_type,
            time: 2.0,
            attack_end: 0.0,
            release_end: 0.0,
        }
    }
    pub fn press(&mut self) {
        if ! self.down { // ignore key repeat
            self.down = true;
            self.active = true;
            self.attack_end = self.attack_duration;
        }
    }

    pub fn release(&mut self) {
        self.down = false;
        self.release_end = self.time + self.release_duration * self.volume;
        self.release_end = self.time;
    }

    pub fn attack_phase(&mut self, freq: i32) {
        self.time = self.time + 1.0 / freq as f32;
        self.volume = (self.time / self.attack_end).min(1.0);
    }

    pub fn release_phase(&mut self, freq: i32) {
        self.time = (self.time + 1.0 / freq as f32).min(self.release_end);
        self.volume = ((self.release_end - self.time) / self.release_end).max(0.0);
        self.active = self.volume == 0.0;
    }

    pub fn update(&mut self, freq: i32, global_volume: f32) -> f32 {
        if self.active {
            match self.down {
                true => self.attack_phase(freq),
                false => self.release_phase(freq)
            }

            // TODO hash-map or jump table me!
            let volume = self.volume * global_volume;
            match self.osc_type {
                Oscilator_Type::SINE => get_sine_sample(self.tone, self.time, volume),
                Oscilator_Type::SAW => get_saw_sample(self.tone, self.time, volume),
                Oscilator_Type::PULSE => get_pulse_sample(self.tone, self.time, volume),
                Oscilator_Type::WHITE_NOISE => get_white_noise_sample(self.tone, self.time, volume),
                Oscilator_Type::SQUARE => get_square_sample(self.tone, self.time, volume),
                _ => 0.0
            }
        } else {
            0.0
        }
    }
}

pub fn get_square_sample(tone: i32, time: f32, volume: f32) -> f32 {
    let mut rng = thread_rng();
    let itone = 1.0 / tone as f32;
    let hitone = itone / 2.0;
    let noise = 0.0 * (rng.next_f32() as f32 * 2.0 - 1.0) * 12.5;
    volume * (if (time % itone) > hitone { noise + (100.0-12.5) } else { noise - (12.0) }) /100.0
}


pub fn get_white_noise_sample(tone: i32, time: f32, volume: f32) -> f32 {
    let mut rng = thread_rng();
    (rng.next_f32() as f32 * 2.0 - 1.0) * volume
}

pub fn get_pulse_sample(tone: i32, time: f32, volume: f32) -> f32 {
    0.0
}

pub fn get_sine_sample(tone: i32, time: f32, volume: f32) -> f32 {
    clamp((PI * 2.0 * time * tone as f32).sin() * volume, -1.0, 1.0)
}

pub fn saw(t: f32) -> f32 {
    (2.0*t-1.0) % 1.0
}

pub fn get_saw_sample(tone: i32, time: f32, volume: f32) -> f32 {
    clamp(saw(tone as f32 * time) * volume, -1.0, 1.0)
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
    }

    pub fn new(freq: i32) -> Self{
        // const TONES : [i32: 8] = [
        //     262, 294, 330, 349,
        //     392, 440, 494, 523
        // ];
        // TODO rework me
        let keys = [
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
            Key::new(100, Oscilator_Type::WHITE_NOISE),
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

    pub fn start_note(&mut self, key_idx: usize) {
        self.keys[key_idx].press()
    }

    pub fn release_note(&mut self, key_idx: usize) {
        self.keys[key_idx].release()
    }
}

fn mix_samples(lhs: f32, rhs: f32) -> f32 {
    lhs + rhs // - if lhs.signum() == rhs.signum() { lhs * rhs.abs() } else { 0.0 }
}

impl AudioCallback for SquareWave {
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
