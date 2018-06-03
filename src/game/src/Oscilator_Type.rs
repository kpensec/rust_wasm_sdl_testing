#[derive(Clone, Copy)]
pub enum Oscilator_Type {
    SINE,
    SAW,
    PULSE, // Stupid this is just a sine, square or tri/saw with proper envelop
    SQUARE,
}
