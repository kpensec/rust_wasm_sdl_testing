
const SCALE_FACTOR: f32 = 1.0594630943592952645618252949463;
const BASE_FREQ: f32 = 440.0;

pub fn get_note_freq(n: i32) -> f32 {
    BASE_FREQ * SCALE_FACTOR.powi(n as i32 - 7)
}


