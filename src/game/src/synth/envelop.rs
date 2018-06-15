
pub struct Envelop {
    pub attack: f32,
    pub decay: f32,
    pub release: f32, 
}

enum KeyState {
    Pressed(f32),
    Released(f32)
}

impl Envelop {
    fn get_amplitude(self, time: f32, keyState: KeyState) -> f32 {
        match keyState {
            Pressed(startTime) => {
                let lifeTime = time - startTime;
                if lifeTime <= self.attackTime {
                    attackTime(lifeTime)
                } else if lifeTime <= self.decayTime {
                    decayPhase(lifeTime)
                } else {
                    sustainPhase(lifeTime)
                }
            }
            Released(endTime) => {
                let dieTime = time - endTime;
                release_phase(lifeTime)
            }
        }
    }
}


fn attack_phase(t:f32) -> f32 {
    0.0
}

fn decay_phase(t:f32) -> f32 {
    0.0
}

fn sustain__phase(t:f32) -> f32 {
    0.0
}

fn release_phase(t:f32) -> f32 {
    0.0
}

