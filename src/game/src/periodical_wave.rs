use std::f32::consts::PI;

#[derive(Clone, Copy)]
pub enum Oscilator_Type {
    SINE,
    SAW,
    SQUARE,
}
// Pulse osc is just another primitive with proper envelop

pub fn sine_wave(time: f32, period: f32) -> f32 {
    (PI * 2.0 * time * period).sin()
}

pub fn saw_wave(time: f32, period: f32) -> f32 {
    (2.0 * (period * time) - 1.0) % 1.0
}

pub fn square_wave(time: f32, period: f32) -> f32 {
    let iperiod = 1.0 / period;
    let hiperiod = iperiod / 2.0;
    if (time % iperiod) > hiperiod {
        1.0
    } else {
        -1.0
    }
}
