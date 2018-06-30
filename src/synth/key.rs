use utils::clamp;
use synth::note;
use synth::instrument::TestInstrument;
#[derive(Clone, Copy)]
pub struct Key {
    pub active: bool,
    pub tone_freq: f32,
    pub volume: f32,
    time: f32,
    key_state: KeyState,
    instrument: TestInstrument,
}

#[derive(Clone, Copy)]
pub enum KeyState {
    Pressed,
    Released(f32),
    Mute,
}

impl Key {
    pub fn new(tone_idx: i32) -> Self {
        Key {
            active: false,
            tone_freq: note::get_note_freq(tone_idx),
            volume: 0.2,
            time: 0.0,
            key_state: KeyState::Mute,
            instrument: TestInstrument::new()
        }
    }

    pub fn _get_state(self) -> KeyState {
        self.key_state
    }

    pub fn press(&mut self) -> () {
        match self.key_state {
            KeyState::Mute => {
                self.key_state = KeyState::Pressed;
                self.active = true;
                self.time = 0.0;
            },
            _ => {}
        }
    }

    pub fn release(&mut self) {
        if let KeyState::Pressed = self.key_state {
            self.key_state = KeyState::Released(self.time);
        }
    }

    pub fn update(&mut self, global_volume: f32, eps: f32) -> f32 {
        let volume = self.volume * global_volume;
        let envelop_amplitude = self.instrument.envelop.get_amplitude(self.time, self.key_state);
        let instrument_sample = self.instrument.get_sample(self.time, self.tone_freq);
        self.time += eps;
        match self.key_state {
            KeyState::Released(..) => {
                if envelop_amplitude == 0.0 {
                    self.key_state = KeyState::Mute;
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
