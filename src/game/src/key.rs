use periodical_wave::{sine_wave, square_wave, saw_wave, Oscilator_Type};
use utils::clamp;

#[derive(Clone, Copy)]
pub struct Key {
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
            let sample = match self.osc_type {
                Oscilator_Type::SINE => sine_wave(self.time, self.tone as f32),
                Oscilator_Type::SAW => saw_wave(self.time, self.tone as f32),
                // Oscilator_Type::WHITE_NOISE => get_white_noise_sample(self.volume, rng.next_f32() as f32)
                Oscilator_Type::SQUARE => square_wave(self.time, self.tone as f32),
            };
            clamp(sample * volume, -1.0, 1.0)
        } else {
            0.0
        }
    }
}
