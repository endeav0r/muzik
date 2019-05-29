use crate::{Sample, SampleType};
use crate::constants::TIME_SCALE;

pub struct TrackSample {
    offset: usize,
    sample: Box<Sample>,
}

impl TrackSample {
    pub fn new<S: Sample + 'static>(offset: usize, sample: S) -> TrackSample {
        TrackSample {
            offset: offset,
            sample: Box::new(sample),
        }
    }

    pub fn offset(&self) -> usize { self.offset * TIME_SCALE }
    pub fn sample(&self) -> &Sample { self.sample.as_ref() }
}

pub struct Track {
    samples: Vec<TrackSample>
}

impl Track {
    pub fn new() -> Track {
        Track { samples: Vec::new() }
    }

    pub fn samples(&self) -> &[TrackSample] { &self.samples }

    pub fn add_track_note(&mut self, track_sample: TrackSample) {
        self.samples.push(track_sample);
    }

    pub fn encode(&self) -> Vec<SampleType> {
        // Create an empty buffer to hold this entire track
        let mut buf: Vec<SampleType> = Vec::new();

        let max_size: usize = self.samples()
            .iter()
            .max_by(|lhs, rhs| (lhs.offset() + lhs.sample().duration()).cmp(&(rhs.offset() + rhs.sample().duration())))
            .map(|track_note| track_note.offset() + track_note.sample().duration())
            .unwrap_or(0);

        buf.resize(max_size, 0);

        println!("max_size={}", max_size);

        // Go through each track note
        for track_sample in self.samples() {
            let sample_buf = track_sample.sample().encode();
            for i in 0..sample_buf.len() {
                let sample = buf[track_sample.offset() + i];
                // println!("{} {}", sample, note_buf[i]);
                let sample = sample + sample_buf[i];
                std::mem::replace(&mut buf[track_sample.offset() + i], sample);
            }
        }

        buf
    }
}