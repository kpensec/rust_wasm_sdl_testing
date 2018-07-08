use synth::periodical_wave::{*};
use synth::envelop::Envelop;
use synth::note::get_note_freq;

#[derive(Clone, Copy)]
struct LowFrequencyOscillator {
    pub amplitude: f32,
    pub freq: f32,
}

impl LowFrequencyOscillator {
    pub fn get(self, time: f32) -> f32 {
        self.amplitude * sine_wave(time, self.freq)
    }
}

#[derive(Clone, Copy)]
pub struct TestInstrument {
    lfo: LowFrequencyOscillator,
    pub envelop: Envelop,
}

impl TestInstrument {
    pub fn new() -> Self {
        TestInstrument {
            envelop: Envelop{
                attack: 0.05,
                decay: 0.15,
                release: 0.1,
                peak_amp: 1.0,
                sustain_amp: 0.5
            },
            lfo: LowFrequencyOscillator{
                amplitude: 0.0000002,
                freq: 1.50,
            }
        }
    }
    pub fn get_sample(self, time: f32, note: i32) -> f32 {
        let tone_1 = get_note_freq(note - 24);
        let tone_2 = get_note_freq(note + 24);
        let tone_3 = get_note_freq(note + 12);
        square_wave(time + self.lfo.get(time), tone_1) * 0.45
            + saw_wave(time + self.lfo.get(2.0*time), tone_2) * 0.45
            + saw_wave(time + self.lfo.get(0.1*time), tone_3) * 0.3
            + noise(time, 0.0) * 0.10
    }
}

