extern crate rand;
// used in noise gen
// use self::rand::{Rng, thread_rng};

use sdl2::audio::{
    AudioStatus,
    AudioDevice,
    AudioCallback
};

use utils::clamp;

use synth::key::Key;



// fn mix_samples(lhs: f32, rhs: f32) -> f32 {
//     lhs + rhs
//     // if lhs != 0.0 && rhs != 0.0 { lhs * rhs } else { rhs + lhs }// - if lhs.signum() == rhs.signum() { lhs * rhs.abs() } else { 0.0 }
// }

// impl AudioCallback for Synthesizer {
//     type Channel = f32;
// 
//     // TODO implement a sound resynch?
//     fn callback(&mut self, out: &mut [f32]) {
//         // Generate a square wave
//         let mut current: f32 = 0.0;
//         let mut first = true;
//         let mut debug_first_sample = true;
//         // let ifreq: f32 = 1.0 / self.freq as f32;
//         for x in out.iter_mut() {
//             if first {
//                 current = 0.0;
//                 for key in self.keys.iter_mut() {
//                     if key.active {
//                         current = mix_samples(current, key.update(self.volume, self.step));
//                     }
//                 }
//                 self.sample_number += 1;
//             }
//             first = ! first;
//             *x = current;
//             debug_first_sample = false;
//         }
//     }
// 
// }

