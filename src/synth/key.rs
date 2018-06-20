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
    pub toneFreq: f32,
    pub volume: f32,
    time: f32,
    keyState: KeyState,
    instrument: Instrument,
}

#[derive(Clone, Copy)]
pub enum KeyState {
    Pressed,
    Released(f32),
    Mute,
}


impl Key {
    pub fn new(toneIdx: i32) -> Self {
        Key {
            active: false,
            toneFreq: note::get_note_freq(toneIdx),
            volume: 0.2,
            time: 0.0,
            keyState: KeyState::Mute,
            instrument: Instrument{
                envelop: Envelop{
                    attack: 0.5,
                    decay: 2.0,
                    release: 1.0,
                    peakAmp: 1.0,
                    sustainAmp: 0.8
                },
                amplitudeLFO: 0.2,
                freqLFO: 1.000,
            },
        }
    }

    pub fn get_state(self) -> KeyState {
        self.keyState
    }

    pub fn press(&mut self) -> () {
        match self.keyState {
            KeyState::Mute => {
                self.keyState = KeyState::Pressed;
                self.active = true;
                self.time = 0.0;
            },
            _ => {}
        }
    }

    pub fn release(&mut self) {
        if let KeyState::Pressed = self.keyState {
            self.keyState = KeyState::Released(self.time);
        }
    }

    pub fn update(&mut self, global_volume: f32, eps: f32) -> f32 {
        let volume = self.volume * global_volume;
        let envelop_amplitude = self.instrument.envelop.get_amplitude(self.time, self.keyState);
        let instrument_sample = self.instrument.get_sample(self.time, self.toneFreq);
        self.time += eps;
        match self.keyState {
            KeyState::Released(..) => {
                if envelop_amplitude == 0.0 {
                    self.keyState = KeyState::Mute;
                    self.active = false;
                }
            },
            _ => {}
        }

        /*match self.osc_type {
        //OscilatorType::Sine     => sine_wave(self.time + 0.01 * saw_wave(self.time, 2.0) + 0.2 * triangle_wave(self.time, 8.0), self.tone as f32)
        //    + 0.25 * sine_wave(self.time + 0.02 * saw_wave(self.time, 5.0), self.tone as f32 * 2.0)
        //    + 0.30 * sine_wave(self.time + 0.2 * saw_wave(self.time, 10.0), self.tone as f32 * 1.5),
        //OscilatorType::Saw      => saw_wave(self.time + 0.05 * saw_wave(self.time, 1.0), self.tone as f32) + 0.2 * sine_wave(self.time + 0.05 * saw_wave(self.time, 1.0), self.tone as f32),
        //OscilatorType::Triangle => triangle_wave(self.time, self.tone as f32),
        //OscilatorType::Square   => square_wave(self.time, self.tone as f32),

        };*/
        clamp(envelop_amplitude * instrument_sample * volume, -1.0, 1.0)
    }
}
