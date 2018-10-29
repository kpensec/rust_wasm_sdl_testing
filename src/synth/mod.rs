
mod key;
mod periodical_wave;
mod noise;
mod envelop;
mod note;
mod instrument;

use utils::clamp;
use synth::key::Key;
use std::vec::Vec;
use sdl2::audio::AudioCallback;

// TODO should be dropped after next TODO implementation!
enum SequencerState {
    Inactive,
    Playing,
    Recording
}

// TODO split Sequencer into a Recorder struct and a Player struct for easier multi channel
//      and to allow play & record behavior
pub struct Sequencer {
    notes: Vec<(usize, f32)>,
    state: SequencerState,
    time: f32,
    current_note: usize,
}

impl Sequencer {
    pub fn new() -> Self {
        Self{
            notes: Vec::<(usize,f32)>::new(),
            state: SequencerState::Inactive,
            time: 0.0,
            current_note: 0,
        }
    }

    pub fn start_record(&mut self) {
        match self.state {
            SequencerState::Inactive => {
                self.time = 0.0;
                self.notes.clear();
                self.state = SequencerState::Recording;
            },
            _ => {}
        }
    }

    pub fn stop_record(&mut self) {
        match self.state {
            SequencerState::Recording => {
                self.state = SequencerState::Inactive;
            },
            _ => {}
        }
    }

    pub fn start_playing(&mut self) {
        match self.state {
            SequencerState::Inactive => {
                self.state = SequencerState::Playing;
                self.current_note = 0;
                self.time = 0.0;
            },
            _ => {}
        }
    }

    pub fn update(&mut self, eps: f32) {
        if self.is_recording() {
            self.time += eps;
        }
    }

    pub fn get_sample(&mut self, _v: f32, _eps: f32) -> f32 {
        const RT : f32 = 0.5;
        let sample = 0.0;
        let mut next_note_idx = self.current_note;
        for idx in self.current_note..self.notes.len() {
            let time = self.notes[idx].1;
            if self.time > time + RT {
                next_note_idx = self.current_note + 1;
            }

        }
        self.current_note =  next_note_idx;
        sample
    }

    pub fn is_recording(&self) -> bool {
        match self.state {
            SequencerState::Recording => true,
            _ => false
        }
    }

    pub fn record(&mut self, key_idx: usize) {
        self.notes.push((key_idx, self.time));
    }

    pub fn is_playing(&self) -> bool {
        match self.state {
            SequencerState::Playing => true,
            _ => false
        }
    }
}

pub struct Synthesizer {
    volume: f32,
    keys: Vec<Key>,
    step: f32,
    active: bool,
    sequencer: Sequencer,
    instrument_idx: usize,
    instruments: Vec<Instrument>,
}

impl Synthesizer {

    pub fn new(playback_freq: i32) -> Self{
        let note_range : Vec<i32> = (0..13).collect();
        let keys = note_range.iter().map(|&x| Key::new(x,0)).collect();

        Synthesizer {
            volume: 1.0,
            keys: keys,
            step: 1.0 / playback_freq as f32,
            active: true,
            sequencer: Sequencer::new(),
            instrument_idx: 0,
            instruments: vec![]
        }
    }

    pub fn record(&mut self) -> () {
        self.sequencer.start_record();
    }

    pub fn is_active(&self) -> bool {
        self.active
    }

    pub fn get_volume(&self) -> f32 {
        self.volume
    }

    pub fn set_volume(&mut self, q: f32) {
        self.volume = clamp(self.volume + q, 0.0, 1.0);
    }

    pub fn set_instrument(&mut self, instrument: Instrument) {
        if self.instruments.len() == 0 {
            self.instruments.push(instrument)
        } else {
            self.instruments[0] = instrument;
        }
    }

   // pub fn get_instrument(&self) -> String {
   //     self.instrument_names[self.instrument_idx].to_string()
   // }

   // pub fn switch_instrument(&mut self, step: i32) {
   //     self.instrument_idx = ((self.instrument_idx as i32 + step) as usize) % self.instrument_names.len();
   // }

    pub fn start_note(&mut self, key_idx: usize) {
        // TODO vec upsert here!
        self.keys[key_idx].press();
        if self.sequencer.is_recording() {
            self.sequencer.record(key_idx);
        }
    }

    pub fn release_note(&mut self, key_idx: usize) {
        // TODO vec update here!
        self.keys[key_idx].release();
        //if self.sequencer.is_recording() {
        //    self.sequencer.record(key_idx);
        //}
    }

    fn blend_sample(lhs: f32, rhs: f32) -> f32 {
        // TODO read a book about sample blending/polyphony!
        lhs + rhs
    }

    fn get_sample(&mut self) -> f32 {
        let mut result = 0.0;
        // TODO this is a map fold...
        for key in self.keys.iter_mut() {
            let new_sample = key.update(self.volume, self.step, &self.instruments);
            result = Self::blend_sample(result, new_sample);
        }

        if self.sequencer.is_playing() {
            result = Self::blend_sample(result, self.sequencer.get_sample(self.volume, self.step))
        }
        result
    }

    pub fn toggle_audio(&mut self) -> (){
        self.active = ! self.active;
    }

}

impl AudioCallback for Synthesizer {
    type Channel = f32;

    fn callback(&mut self, output: &mut [f32]) {
        let mut chan = 0;
        if self.is_active() {
            let mut sample = 0.0;
            for x in output.iter_mut() {
                if chan == 0 {
                    sample = self.get_sample();
                }
                chan = chan + 1;
                if chan > 2 {
                    chan = 0
                }
                *x = sample;
            }
        } else {
            // nullify buffer
            for x in output.iter_mut() {
                *x = 0.0;
            }
        }
    }
}

pub use self::instrument::Instrument;
