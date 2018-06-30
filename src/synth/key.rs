use utils::clamp;
use synth::instrument::TestInstrument;

#[derive(Clone, Copy)]
pub struct Key {
    pub active: bool,
    pub note_idx: i32,
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
    pub fn new(note_idx: i32) -> Self {
        Key {
            active: false,
            note_idx: note_idx,
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
        let instrument_sample = self.instrument.get_sample(self.time, self.note_idx);
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
