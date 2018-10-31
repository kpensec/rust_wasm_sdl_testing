use synth::periodical_wave::{*};
use synth::envelop::Envelop;
use synth::note::get_note_freq;
use utils::Unit;

#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
pub struct Oscillator {
    osc_func: usize,
    osc_amp: Unit,
    osc_note_offset: i32, // change this to freq offset?
    lfo_func: usize,
    lfo_amp: Unit,
    lfo_freq: Unit,
}

static OSC_FUNCS: [fn(Unit, Unit) -> Unit; 4] = [
    sine_wave,
    saw_wave,
    square_wave,
    noise
];

impl Oscillator {
    pub fn get_sample(self, time: Unit, note: i32) -> Unit {
        let note_freq = get_note_freq(note + self.osc_note_offset);
        let lfo_shift = OSC_FUNCS[self.lfo_func](time, self.lfo_freq) * self.lfo_amp;
        OSC_FUNCS[self.osc_func](time + lfo_shift, note_freq) * self.osc_amp
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Instrument(Vec<Oscillator>);

impl Instrument {
    pub fn get_sample(self, time: Unit, note: i32) -> Unit {
        self.0.into_iter()
            .map(|osc| osc.get_sample(time, note))
            .fold(0.0, |acc, sample| acc + sample)
    }
}

// need to remove below code?

#[derive(Clone, Copy)]
struct LowFrequencyOscillator {
    pub amplitude: Unit,
    pub freq: Unit,
}

impl LowFrequencyOscillator {
    pub fn get(self, time: Unit) -> Unit {
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
            },
        }
    }

    pub fn get_sample(&mut self, time: Unit, note: i32) -> Unit {
        let tone_1 = get_note_freq(note - 1);
        let tone_2 = get_note_freq(note + 15);
        let tone_3 = get_note_freq(note);
        square_wave(time + self.lfo.get(time), tone_1) * 0.45
            + saw_wave(time + self.lfo.get(2.0*time), tone_2) * 0.45
            + saw_wave(time + self.lfo.get(0.1*time), tone_3) * 0.3
            + noise(time, 0.0) * 0.10
    }
}

