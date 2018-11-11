use synth::periodical_wave::{*};
use synth::note::get_note_freq;
use utils::Unit;

#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
pub struct Oscillator {
    pub osc_func: usize,
    pub osc_amp: Unit,
    pub osc_note_offset: i32, // change this to freq offset?
    pub lfo_func: usize,
    pub lfo_amp: Unit,
    pub lfo_freq: Unit,
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
    pub fn new() -> Self {
        Instrument{0: vec![]}
    }
    pub fn get_sample(self, time: Unit, note: i32) -> Unit {
        self.0.into_iter()
            .map(|osc| osc.get_sample(time, note))
            .fold(0.0, |acc, sample| acc + sample)
    }
    pub fn get_vec_mut(&mut self) -> &mut Vec<Oscillator> {
        &mut self.0
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



