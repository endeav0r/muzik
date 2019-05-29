pub const SAMPLE_RATE: usize = 32768;
pub const NUM_CHANNELS: usize = 2;
pub const BITS_PER_SAMPLE: usize = 16;

pub const TIME_SCALE: usize = 1500;

pub fn hz_freq(hz: f64) -> f64 {
    (SAMPLE_RATE as f64) * (1.0 / hz)
}

pub const A1_HZ: f64 = 55.00;
pub const A1S_HZ: f64 = 58.27;
pub const B1_HZ: f64 = 61.74;
pub const C2_HZ: f64 = 65.41;
pub const C2S_HZ: f64 = 69.30;
pub const D2_HZ: f64 = 73.42;
pub const D2S_HZ: f64 = 77.78;
pub const E2_HZ: f64 = 82.41;
pub const F2_HZ: f64 = 87.31;
pub const F2S_HZ: f64 = 92.50;
pub const G2_HZ: f64 = 98.00;
pub const G2S_HZ: f64 = 103.83;
pub const A2_HZ: f64 = 110.00;
pub const A2S_HZ: f64 = 116.54;
pub const B2_HZ: f64 = 123.47;
pub const C3_HZ: f64 = 130.81;
pub const C3S_HZ: f64 = 138.59;
pub const D3_HZ: f64 = 146.83;
pub const D3S_HZ: f64 = 155.56;
pub const E3_HZ: f64 = 164.81;
pub const F3_HZ: f64 = 174.61;
pub const F3S_HZ: f64 = 185.00;
pub const G3_HZ: f64 = 196.00;
pub const G3S_HZ: f64 = 207.65;
pub const A3_HZ: f64 = 220.0;
pub const A3S_HZ: f64 = 233.08;
pub const B3_HZ: f64 = 246.94;
pub const C4_HZ: f64 = 261.63;
pub const C4S_HZ: f64 = 277.18;
pub const D4_HZ: f64 = 293.66;
pub const D4S_HZ: f64 = 311.13;
pub const E4_HZ: f64 = 329.63;
pub const F4_HZ: f64 = 349.24;
pub const F4S_HZ: f64 = 369.99;
pub const G4_HZ: f64 = 392.00;
pub const G4S_HZ: f64 = 415.30;
pub const A4_HZ: f64 = 440.0;
pub const A4S_HZ: f64 = 466.16;
pub const B4_HZ: f64 = 493.88;
pub const C5_HZ: f64 = 523.25;