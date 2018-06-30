use synth::periodical_wave::{*};
use synth::envelop::Envelop;

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
                decay: 0.2,
                release: 0.1,
                peak_amp: 1.0,
                sustain_amp: 0.8
            },
            lfo: LowFrequencyOscillator{
                amplitude: 0.0000002,
                freq: 1.50,
            }
        }
    }
    pub fn get_sample(self, time: f32, tone_freq: f32) -> f32 {
         square_wave(time, 2.0 * tone_freq)
             + square_wave(time, tone_freq) * 0.5
             + square_wave(time, tone_freq*0.8) * 1.5
             + sine_wave(time + self.lfo.get(time), tone_freq*2.777) * 0.8
        // + 0.2 * sine_wave(time + self.amplitude_lfo * sine_wave(time, self.freq_lfo), tone_freq*1.2)
        //+ 0.2 * sine_wave(time + self.amplitude_lfo * sine_wave(time, self.freq_lfo), tone_freq*0.8)
        //0.2 * saw_wave(time, tone_freq)
        //    + 0.2 * square_wave(time + self.lfo.get(time), tone_freq*0.7)
        //    + 0.1 * square_wave(time + self.lfo.get(time), tone_freq*2.0)
        //    + 0.15 * square_wave(time + self.lfo.get(time), tone_freq*2.5)
    }
}
