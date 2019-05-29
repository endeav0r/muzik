pub mod constants;

mod note;
mod track;
mod wav;

pub use note::{Note, WaveType};
pub use track::{Track, TrackSample};
pub use wav::Wav;

pub use constants::hz_freq;

pub type SampleType = i16;


pub trait Sample {
    fn encode(&self) -> Vec<SampleType>;
    fn duration(&self) -> usize { self.encode().len() }
}

pub struct Attack {
    sample: Box<Sample>
}

impl Attack {
    pub fn new<S: Sample + 'static>(sample: S) -> Attack {
        Attack {
            sample: Box::new(sample)
        }
    }

    pub fn encode(&self) -> Vec<SampleType> {
        let mut samples = self.sample.encode();

        let samples_len = samples.len();

        for i in 0..samples_len {
            let adjustment: f64 = if i < samples_len / 4 {
                let glide = (i as f64) / (samples_len as f64);
                glide + 0.75
            }
            else {
                0.75
            };
            let sample = samples[i] as f64;
            std::mem::replace(&mut samples[i], (sample * adjustment) as SampleType);
        }

        samples
    }
}

impl Sample for Attack {
    fn encode(&self) -> Vec<SampleType> { self.encode() }
}