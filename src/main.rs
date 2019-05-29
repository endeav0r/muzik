#![allow(dead_code)]
use std::fs::File;
use std::io::Write;

use muzik::constants::*;
use muzik::{Attack, hz_freq, Track, TrackSample, Note, Wav, WaveType};

fn main() -> Result<(), Box<std::error::Error>> {
    let mut track = Track::new();

    use WaveType::*;

    // track.add_track_note(TrackNote::new(0, Note::new(16, 128.0, 2048.0, SawTooth)));
    // track.add_track_note(TrackNote::new(16, Note::new(16, 128.0, 2048.0, Sine)));
    // track.add_track_note(TrackNote::new(32, Note::new(16, 128.0, 2048.0, Triangle)));

    fn saw(track: &mut Track, offset: usize, duration: usize, frequency: f64, amplitude: f64, wave_type: WaveType) {
        let note = Note::new(duration, frequency, amplitude, wave_type);
        let attack = Attack::new(note);
        track.add_track_note(TrackSample::new(offset, attack));
    }

    let g3 = hz_freq(G3_HZ);
    let a3s = hz_freq(A3S_HZ);
    let b3 = hz_freq(B3_HZ);
    let c4 = hz_freq(C4_HZ);
    let g4 = hz_freq(G4_HZ);
    let d4 = hz_freq(D4_HZ);
    let d4s = hz_freq(D4S_HZ);
    let e4 = hz_freq(E4_HZ);
    let f4 = hz_freq(F4_HZ);

    let melody_wave_type = Triangle;

    saw(&mut track, 0, 16, g4, 2048.0, melody_wave_type);
    saw(&mut track, 16, 16, c4, 2048.0, melody_wave_type);
    saw(&mut track, 32, 8, d4s, 2048.0, melody_wave_type);
    saw(&mut track, 40, 8, f4, 2048.0, melody_wave_type);

    saw(&mut track, 0, 96, hz_freq(G3_HZ), 2048.0, Sine);
    saw(&mut track, 0, 96, hz_freq(C3_HZ), 2048.0, Sine);

    saw(&mut track, 48, 16, g4, 2048.0, melody_wave_type);
    saw(&mut track, 64, 16, c4, 2048.0, melody_wave_type);
    saw(&mut track, 80, 8, d4s, 2048.0, melody_wave_type);
    saw(&mut track, 88, 8, f4, 2048.0, melody_wave_type);

    saw(&mut track, 96, 16, g4, 2048.0, melody_wave_type);
    saw(&mut track, 112, 16, c4, 2048.0, melody_wave_type);
    saw(&mut track, 128, 8, e4, 2048.0, melody_wave_type);
    saw(&mut track, 136, 8, f4, 2048.0, melody_wave_type);

    saw(&mut track, 96, 96, hz_freq(G3_HZ), 2048.0, Sine);
    saw(&mut track, 96, 96, hz_freq(C3_HZ), 2048.0, Sine);

    saw(&mut track, 144, 16, g4, 2048.0, melody_wave_type);
    saw(&mut track, 160, 16, c4, 2048.0, melody_wave_type);
    saw(&mut track, 176, 8, e4, 2048.0, melody_wave_type);
    saw(&mut track, 184, 8, f4, 2048.0, melody_wave_type);

    saw(&mut track, 192, 48, g4, 2048.0, melody_wave_type);
    saw(&mut track, 240, 48, c4, 2048.0, melody_wave_type);

    saw(&mut track, 192, 96, hz_freq(G3_HZ), 2048.0, Sine);
    saw(&mut track, 192, 96, hz_freq(C3_HZ), 2048.0, Sine);

    saw(&mut track, 288, 8, c4, 2048.0, melody_wave_type);
    saw(&mut track, 288, 8, d4s, 2048.0, melody_wave_type);
    saw(&mut track, 296, 8, f4, 2048.0, melody_wave_type);
    saw(&mut track, 304, 32, g4, 2048.0, melody_wave_type);

    saw(&mut track, 288, 96, hz_freq(G3_HZ), 2048.0, Sine);
    saw(&mut track, 288, 96, hz_freq(C3_HZ), 2048.0, Sine);

    saw(&mut track, 334, 32, c4, 2048.0, melody_wave_type);
    saw(&mut track, 366, 8, d4s, 2048.0, melody_wave_type);
    saw(&mut track, 374, 8, f4, 2048.0, melody_wave_type);

    saw(&mut track, 382, 16, a3s, 2048.0, melody_wave_type);
    saw(&mut track, 382, 16, d4, 2048.0, melody_wave_type);
    saw(&mut track, 398, 16, g3, 2048.0, melody_wave_type);
    saw(&mut track, 414, 8, a3s, 2048.0, melody_wave_type);
    saw(&mut track, 422, 8, c4, 2048.0, melody_wave_type);

    saw(&mut track, 382, 96, hz_freq(D4_HZ), 2048.0, Sine);
    saw(&mut track, 382, 96, hz_freq(G3_HZ), 2048.0, Sine);

    saw(&mut track, 430, 16, d4, 2048.0, melody_wave_type);
    saw(&mut track, 446, 16, g3, 2048.0, melody_wave_type);
    saw(&mut track, 462, 8, a3s, 2048.0, melody_wave_type);
    saw(&mut track, 470, 8, c4, 2048.0, melody_wave_type);

    let bar = 478;

    saw(&mut track, bar, 16, a3s, 2048.0, melody_wave_type);
    saw(&mut track, bar, 16, d4, 2048.0, melody_wave_type);
    saw(&mut track, bar + 16, 16, g3, 2048.0, melody_wave_type);
    saw(&mut track, bar + 32, 8, a3s, 2048.0, melody_wave_type);
    saw(&mut track, bar + 40, 8, c4, 2048.0, melody_wave_type);

    saw(&mut track, bar, 96, hz_freq(D4_HZ), 2048.0, Sine);
    saw(&mut track, bar, 96, hz_freq(G3_HZ), 2048.0, Sine);

    let bar = 526;

    saw(&mut track, bar, 16, d4, 2048.0, melody_wave_type);
    saw(&mut track, bar + 16, 16, g3, 2048.0, melody_wave_type);
    saw(&mut track, bar + 32, 16, a3s, 2048.0, melody_wave_type);

    let bar = bar + 48;

    saw(&mut track, bar, 48, f4, 2048.0, melody_wave_type);

    saw(&mut track, bar, 96, hz_freq(F3_HZ), 2048.0, Sine);
    saw(&mut track, bar, 96, hz_freq(A2S_HZ), 2048.0, Sine);

    let bar = bar + 48;

    saw(&mut track, bar, 48, a3s, 2048.0, melody_wave_type);

    let bar = bar + 48;

    saw(&mut track, bar, 8, a3s, 2048.0, melody_wave_type);
    saw(&mut track, bar, 8, d4s, 2048.0, melody_wave_type);
    saw(&mut track, bar + 8, 8, d4, 2048.0, melody_wave_type);
    saw(&mut track, bar + 16, 32, f4, 2048.0, melody_wave_type);

    saw(&mut track, bar, 96, hz_freq(F3_HZ), 2048.0, Sine);
    saw(&mut track, bar, 96, hz_freq(A2S_HZ), 2048.0, Sine);

    let bar = bar + 48;

    saw(&mut track, bar, 32, a3s, 2048.0, melody_wave_type);
    saw(&mut track, bar + 32, 8, d4s, 2048.0, melody_wave_type);
    saw(&mut track, bar + 40, 8, d4, 2048.0, melody_wave_type);

    let bar = bar + 48;

    saw(&mut track, bar, 16, c4, 2048.0, melody_wave_type);
    saw(&mut track, bar, 16, hz_freq(A3_HZ), 2048.0, melody_wave_type);
    saw(&mut track, bar + 16, 16, hz_freq(F3_HZ), 2048.0, melody_wave_type);
    saw(&mut track, bar + 32, 8, hz_freq(G3S_HZ), 2048.0, melody_wave_type);
    saw(&mut track, bar + 40, 8, hz_freq(A3S_HZ), 2048.0, melody_wave_type);

    saw(&mut track, bar, 96, hz_freq(C4_HZ), 2048.0, Sine);
    saw(&mut track, bar, 96, hz_freq(F3_HZ), 2048.0, Sine);

    let bar = bar + 48;

    saw(&mut track, bar, 16, c4, 2048.0, melody_wave_type);
    saw(&mut track, bar + 16, 16, hz_freq(F3_HZ), 2048.0, melody_wave_type);
    saw(&mut track, bar + 32, 8, hz_freq(G3S_HZ), 2048.0, melody_wave_type);
    saw(&mut track, bar + 40, 8, hz_freq(A3S_HZ), 2048.0, melody_wave_type);

    let bar = bar + 48;

    saw(&mut track, bar, 16, c4, 2048.0, melody_wave_type);
    saw(&mut track, bar, 16, hz_freq(A3_HZ), 2048.0, melody_wave_type);
    saw(&mut track, bar + 16, 16, hz_freq(F3_HZ), 2048.0, melody_wave_type);
    saw(&mut track, bar + 32, 8, hz_freq(G3S_HZ), 2048.0, melody_wave_type);
    saw(&mut track, bar + 40, 8, hz_freq(A3S_HZ), 2048.0, melody_wave_type);

    saw(&mut track, bar, 96, hz_freq(C4_HZ), 2048.0, Sine);
    saw(&mut track, bar, 96, hz_freq(F3_HZ), 2048.0, Sine);

    let bar = bar + 48;

    saw(&mut track, bar, 16, c4, 2048.0, melody_wave_type);
    saw(&mut track, bar + 16, 16, hz_freq(F3_HZ), 2048.0, melody_wave_type);
    saw(&mut track, bar + 32, 16, hz_freq(G3S_HZ), 2048.0, melody_wave_type);

    let bar = bar + 48;

    saw(&mut track, bar, 48, g4, 2048.0, melody_wave_type);

    saw(&mut track, bar, 32, g3, 2048.0, melody_wave_type);
    saw(&mut track, bar, 32, hz_freq(C3_HZ), 2048.0, melody_wave_type);
    saw(&mut track, bar + 32, 16, g3, 2048.0, melody_wave_type);
    saw(&mut track, bar + 32, 16, hz_freq(C3_HZ), 2048.0, melody_wave_type);

    let bar = bar + 48;

    saw(&mut track, bar, 48, c4, 2048.0, melody_wave_type);

    saw(&mut track, bar, 32, g3, 2048.0, melody_wave_type);
    saw(&mut track, bar, 32, hz_freq(C3_HZ), 2048.0, melody_wave_type);
    saw(&mut track, bar + 32, 16, g3, 2048.0, melody_wave_type);
    saw(&mut track, bar + 32, 16, hz_freq(C3_HZ), 2048.0, melody_wave_type);

    let bar = bar + 48;

    saw(&mut track, bar, 8, c4, 2048.0, melody_wave_type);
    saw(&mut track, bar, 8, d4s, 2048.0, melody_wave_type);
    saw(&mut track, bar + 8, 8, f4, 2048.0, melody_wave_type);
    saw(&mut track, bar + 16, 32, g4, 2048.0, melody_wave_type);

    saw(&mut track, bar, 32, g3, 2048.0, melody_wave_type);
    saw(&mut track, bar, 32, hz_freq(C3_HZ), 2048.0, melody_wave_type);
    saw(&mut track, bar + 32, 16, g3, 2048.0, melody_wave_type);
    saw(&mut track, bar + 32, 16, hz_freq(C3_HZ), 2048.0, melody_wave_type);

    let bar = bar + 48;

    saw(&mut track, bar, 32, c4, 2048.0, melody_wave_type);
    saw(&mut track, bar + 32, 8, d4s, 2048.0, melody_wave_type);
    saw(&mut track, bar + 40, 8, f4, 2048.0, melody_wave_type);

    saw(&mut track, bar, 32, g3, 2048.0, melody_wave_type);
    saw(&mut track, bar, 32, hz_freq(C3_HZ), 2048.0, melody_wave_type);
    saw(&mut track, bar + 32, 16, g3, 2048.0, melody_wave_type);
    saw(&mut track, bar + 32, 16, hz_freq(C3_HZ), 2048.0, melody_wave_type);

    let bar = bar + 48;

    saw(&mut track, bar, 16, b3, 2048.0, melody_wave_type);
    saw(&mut track, bar, 16, hz_freq(D4_HZ), 2048.0, melody_wave_type);
    saw(&mut track, bar + 16, 16, g3, 2048.0, melody_wave_type);
    saw(&mut track, bar + 32, 8, a3s, 2048.0, melody_wave_type);
    saw(&mut track, bar + 40, 8, c4, 2048.0, melody_wave_type);

    saw(&mut track, bar, 32, hz_freq(G2_HZ), 2048.0, melody_wave_type);
    saw(&mut track, bar, 32, hz_freq(C3_HZ), 2048.0, melody_wave_type);
    saw(&mut track, bar + 32, 16, hz_freq(G2_HZ), 2048.0, melody_wave_type);
    saw(&mut track, bar + 32, 16, hz_freq(C3_HZ), 2048.0, melody_wave_type);

    let bar = bar + 48;

    saw(&mut track, bar, 16, hz_freq(D4_HZ), 2048.0, melody_wave_type);
    saw(&mut track, bar + 16, 16, g3, 2048.0, melody_wave_type);
    saw(&mut track, bar + 32, 8, a3s, 2048.0, melody_wave_type);
    saw(&mut track, bar + 40, 8, c4, 2048.0, melody_wave_type);

    saw(&mut track, bar, 32, hz_freq(G2_HZ), 2048.0, melody_wave_type);
    saw(&mut track, bar, 32, hz_freq(C3_HZ), 2048.0, melody_wave_type);
    saw(&mut track, bar + 32, 16, hz_freq(G2_HZ), 2048.0, melody_wave_type);
    saw(&mut track, bar + 32, 16, hz_freq(C3_HZ), 2048.0, melody_wave_type);

    let bar = bar + 48;

    saw(&mut track, bar, 16, b3, 2048.0, melody_wave_type);
    saw(&mut track, bar, 16, hz_freq(D4_HZ), 2048.0, melody_wave_type);
    saw(&mut track, bar + 16, 16, g3, 2048.0, melody_wave_type);
    saw(&mut track, bar + 32, 8, a3s, 2048.0, melody_wave_type);
    saw(&mut track, bar + 40, 8, c4, 2048.0, melody_wave_type);

    saw(&mut track, bar, 32, hz_freq(G2_HZ), 2048.0, melody_wave_type);
    saw(&mut track, bar, 32, hz_freq(C3_HZ), 2048.0, melody_wave_type);
    saw(&mut track, bar + 32, 16, hz_freq(G2_HZ), 2048.0, melody_wave_type);
    saw(&mut track, bar + 32, 16, hz_freq(C3_HZ), 2048.0, melody_wave_type);

    let bar = bar + 48;

    saw(&mut track, bar, 16, hz_freq(D4_HZ), 2048.0, melody_wave_type);
    saw(&mut track, bar + 16, 16, g3, 2048.0, melody_wave_type);
    saw(&mut track, bar + 32, 16, a3s, 2048.0, melody_wave_type);

    saw(&mut track, bar, 32, hz_freq(G2_HZ), 2048.0, melody_wave_type);
    saw(&mut track, bar, 32, hz_freq(C3_HZ), 2048.0, melody_wave_type);
    saw(&mut track, bar + 32, 16, hz_freq(G2_HZ), 2048.0, melody_wave_type);
    saw(&mut track, bar + 32, 16, hz_freq(C3_HZ), 2048.0, melody_wave_type);

    let wav = Wav::new(track.encode());

    let mut file = File::create("/tmp/test.wav")?;
    file.write_all(&wav.encode()?)?;

    Ok(())
}
