use std::fmt;
use synth::key::KeyState;

#[derive(Clone, Copy)]
pub struct Envelop {
    pub attack: f32,
    pub decay: f32,
    pub release: f32,
    pub peak_amp: f32,
    pub sustain_amp: f32,
}

impl fmt::Display for Envelop {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "attack: {}, decay: {}, release: {}", self.attack, self.decay, self.release)
    }
}


impl Envelop {
    pub fn get_amplitude(self, time: f32, key_state: KeyState) -> f32 {
        match key_state {
            KeyState::Pressed => {
                let life_time = time;
                if life_time < self.attack {
                    life_time / self.attack * self.peak_amp
                } else if life_time < self.decay {
                    let m = (self.sustain_amp - self.peak_amp)/(self.decay - self.attack);
                    let b = self.peak_amp - m * self.attack;
                    m * life_time + b
                    //(1.0 - (life_time - self.attack) / self.decay) * self.sustain_amp
                } else {
                    self.sustain_amp
                }
            }
            KeyState::Released(end_time) => {
                if time < self.decay {
                    let m = (self.sustain_amp - self.peak_amp)/(self.decay - self.attack);
                    let b = self.peak_amp - m * self.attack;
                    m * time + b

                } else {
                    let m = (-self.sustain_amp)/(self.release);
                    let b = self.sustain_amp - m * end_time;
                    let r = m * time + b;
                    if r < 0.0 { 0.0 } else { r }

                }
            },
            KeyState::Mute => {
                0.0
            }
        }
    }
}

