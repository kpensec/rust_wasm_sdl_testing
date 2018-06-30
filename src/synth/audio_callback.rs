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

