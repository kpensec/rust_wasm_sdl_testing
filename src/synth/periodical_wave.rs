extern crate rand;
use std::f32::consts::PI;

#[derive(Clone, Copy)]
pub enum _OscilatorType {
    _Sine,
    _Saw,
    _Square,
    _Triangle,
    _Noise
}

pub fn sine_wave(time: f32, freq: f32) -> f32 {
    (PI * 2.0 * time * freq).sin()
}

pub fn saw_wave(time: f32, freq: f32) -> f32 {
    (2.0 * (freq * time) - 1.0) % 1.0
}

pub fn square_wave(time: f32, freq: f32) -> f32 {
    if sine_wave(time, freq) < 0.0 { -1.0 } else { 1.0 }
}

pub fn noise(_time: f32, _freq: f32) -> f32 {
    use self::rand::{Rng, thread_rng};
    let mut rng = thread_rng();
    rng.gen::<f32>() * 2.0 - 1.0
}
