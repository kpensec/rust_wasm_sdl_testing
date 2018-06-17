use synth::periodical_wave::{*};
use utils::clamp;

use synth::note;
use synth::envelop::Envelop;

#[derive(Clone, Copy)]
pub struct Instrument {
    pub envelop: Envelop,
    pub amplitudeLFO: f32,
    pub freqLFO: f32
}

impl Instrument {
    pub fn get_sample(self, time: f32, toneFreq: f32) -> f32 {
        saw_wave(time + self.amplitudeLFO * sine_wave(time, self.freqLFO), toneFreq)
    }
}

#[derive(Clone, Copy)]
pub struct Key {
    pub active: bool,
    pub down: bool,
    pub attack_duration: f32,
    pub release_duration: f32,
    pub toneFreq: f32,
    pub volume: f32,
    osc_type: OscilatorType,
    attack_end: f32,
    release_end: f32,
    time: f32,
    keyState: KeyState,
    instrument: Instrument,
}

#[derive(Clone, Copy)]
pub enum KeyState {
    Pressed(f32),
    Released(f32),
    Mute,
}


impl Key {
    pub fn new(toneIdx: i32, osc_type: OscilatorType) -> Self {
        Key {
            active: false,
            down: false,
            attack_duration: 1.0,
            release_duration: 1.5,
            toneFreq: note::get_note_freq(toneIdx),
            volume: 0.0,

            osc_type: osc_type,
            time: 2.0,
            attack_end: 0.0,
            release_end: 0.0,
            keyState: KeyState::Mute,
            instrument: Instrument{
                envelop: Envelop{
                    attack: 0.5,
                    decay: 0.8,
                    release: 1.0,
                    peakAmp: 1.0,
                    sustainAmp: 0.5
                },
                amplitudeLFO: 0.05,
                freqLFO: 1.0,
            },

        }
    }

    pub fn get_state(self) -> KeyState {
        self.keyState
    }


    pub fn press(&mut self, time: f32) {
        match self.keyState {
            KeyState::Mute => {self.keyState = KeyState::Pressed(time);},
            _ => {}
        }
    }

    pub fn release(&mut self, time: f32) {
        if let KeyState::Pressed(pressedTime) = self.keyState {
            self.keyState = KeyState::Released(time)
        }
        // self.down = false;
        // self.release_end = self.time + self.release_duration * self.volume;
        // self.release_end = self.time;
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
            // TODO find a way to structure with modulation and envelop
            let volume = self.volume * global_volume;
            let sample = 0.0; /*match self.osc_type {
                //OscilatorType::Sine     => sine_wave(self.time + 0.01 * saw_wave(self.time, 2.0) + 0.2 * triangle_wave(self.time, 8.0), self.tone as f32)
                //    + 0.25 * sine_wave(self.time + 0.02 * saw_wave(self.time, 5.0), self.tone as f32 * 2.0)
                //    + 0.30 * sine_wave(self.time + 0.2 * saw_wave(self.time, 10.0), self.tone as f32 * 1.5),
                //OscilatorType::Saw      => saw_wave(self.time + 0.05 * saw_wave(self.time, 1.0), self.tone as f32) + 0.2 * sine_wave(self.time + 0.05 * saw_wave(self.time, 1.0), self.tone as f32),
                //OscilatorType::Triangle => triangle_wave(self.time, self.tone as f32),
                //OscilatorType::Square   => square_wave(self.time, self.tone as f32),
                
            };*/
            clamp(sample * volume, -1.0, 1.0)
        } else {
            0.0
        }
    }
}
