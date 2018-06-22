use std::f32::consts::PI;

#[derive(Clone, Copy)]
pub enum OscilatorType {
    Sine,
    Saw,
    Square,
    Triangle,
    Noise
}
// Pulse osc is just another primitive with proper envelop

pub fn sine_wave(time: f32, freq: f32) -> f32 {
    (PI * 2.0 * time * freq).sin()
}

pub fn saw_wave(time: f32, freq: f32) -> f32 {
    (2.0 * (freq * time) - 1.0) % 1.0
}

pub fn square_wave(time: f32, freq: f32) -> f32 {
    let ifreq = 1.0 / freq;
    let hifreq = ifreq / 2.0;
    if (time % ifreq) > hifreq {
        1.0
    } else {
        -1.0
    }
}

pub fn triangle_wave(time: f32, freq: f32) -> f32 {
    square_wave(time, freq) * saw_wave(time, freq)
}

use synth::rand;
pub fn noise_wave(_time: f32, _freq: f32) -> f32 {
    // let mut rng = rand::thread_rng;
    rand::random::<f32>()
}
