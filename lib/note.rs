use crate::constants::TIME_SCALE;
use crate::{Sample, SampleType};

#[derive(Clone, Copy, Debug)]
pub enum WaveType {
    SawTooth,
    Sine,
    Triangle
}

#[derive(Clone, Debug)]
pub struct Note {
    duration: usize,
    frequency: f64,
    amplitude: f64,
    wave_type: WaveType,
}

impl Note {
    pub fn new(duration: usize, frequency: f64, amplitude: f64, wave_type: WaveType) -> Note {
        Note {
            duration: duration,
            frequency: frequency,
            amplitude: amplitude,
            wave_type: wave_type
        }
    }

    pub fn frequency(&self) -> f64 { self.frequency }
    pub fn duration(&self) -> usize { self.duration * TIME_SCALE }
    pub fn amplitude(&self) -> f64 { self.amplitude }
    pub fn wave_type(&self) -> &WaveType { &self.wave_type }

    pub fn encode(&self) -> Vec<SampleType> {
        let mut samples: Vec<SampleType> = Vec::new();
        for i in 0..self.duration() {
            let phase = (i as f64 % self.frequency()) / self.frequency();
            match self.wave_type() {
                WaveType::SawTooth => {
                    samples.push((self.amplitude() * phase - (self.amplitude() / 2.0)) as SampleType);
                },
                WaveType::Sine => {
                    let sin = (std::f64::consts::PI * 2.0 * phase).sin();
                    samples.push((sin * (self.amplitude() * 0.5)) as SampleType);
                },
                WaveType::Triangle => {
                    let half_amplitude = self.amplitude() * 0.5;
                    if phase < 0.25 {
                        samples.push((half_amplitude * phase * 4.0) as SampleType);
                    }
                    else if phase < 0.5 {
                        samples.push((half_amplitude - (half_amplitude * (phase - 0.25) * 4.0)) as SampleType);
                    }
                    else if phase < 0.75 {
                        samples.push((0.0 - (half_amplitude * (phase - 0.50) * 4.0)) as SampleType);
                    }
                    else {
                        samples.push(((0.0 - half_amplitude) + (half_amplitude * (phase - 0.75) * 4.0)) as SampleType);
                    }
                }
            }
        }
        samples
    }
}

impl Sample for Note {
    fn encode(&self) -> Vec<SampleType> {
        self.encode()
    }
}