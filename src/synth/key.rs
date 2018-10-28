use utils::clamp;
use synth::instrument::Instrument; // TODO rm this coupling!
use synth::envelop::Envelop;

#[derive(Clone)]
pub struct Key {
    pub active: bool,
    pub note_idx: i32,
    pub volume: f32,
    time: f32,
    key_state: KeyState,
    instrument: Instrument,
}

#[derive(Clone)]
pub enum KeyState {
    Pressed,
    Released(f32),
    Mute,
}

impl Key {
    pub fn new(note_idx: i32, instrument: Instrument) -> Self {
        Key {
            active: false,
            note_idx: note_idx,
            volume: 0.2,
            time: 0.0,
            key_state: KeyState::Mute,
            instrument: instrument
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
        let envelop = Envelop {
                attack: 0.05,
                decay: 0.15,
                release: 0.1,
                peak_amp: 1.0,
                sustain_amp: 0.5
        };
        let volume = self.volume * global_volume;
        let envelop_amplitude = envelop.get_amplitude(self.time, self.key_state.clone());
        let instrument_sample = self.clone().instrument.get_sample(self.time, self.note_idx);
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

        clamp(envelop_amplitude * instrument_sample * volume, -1.0, 1.0)
    }
}
