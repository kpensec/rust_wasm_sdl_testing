
struct Noise {
    pub padding: f32,
    amplitude: f32,
    amp_coeff: f32,
};

impl Noise {
    pub fn new(amp: f32, padding: f32) {
        let res = Noise{padding: padding}
        res.set_amp(amp);
        res
    }
    pub fn set_amp(&mut self, amplitude: f32) {
        self.amplitude = amplitude;
        self.amp_coeff = amplitude - amplitude / 2.0;
    }
    pub fn get_sample(self, rng_number: f32) {
        rng_number * self.amp_coeff - self.padding
    }
}
