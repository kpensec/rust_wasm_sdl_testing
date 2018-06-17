use std::fmt;
use synth::key::KeyState;

#[derive(Clone, Copy)]
pub struct Envelop {
    pub attack: f32,
    pub decay: f32,
    pub release: f32, 
    pub peakAmp: f32,
    pub sustainAmp: f32,
}

impl fmt::Display for Envelop {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "attack: {}, decay: {}, release: {}", self.attack, self.decay, self.release)
    }
}


impl Envelop {
    fn get_amplitude(self, time: f32, keyState: KeyState) -> f32 {
        match keyState {
            KeyState::Pressed(startTime) => {
                let lifeTime = time - startTime;
                if lifeTime < self.attack {
                    lifeTime / self.attack * self.peakAmp
                } else if lifeTime < self.decay {
                    (1.0 - (lifeTime - self.attack) / self.decay) * self.sustainAmp
                } else {
                    self.sustainAmp
                }
            }
            KeyState::Released(endTime) => {
                let dieTime = time - endTime;
                // release_phase(dieTime);
                    0.0
            }
        }
    }
}

