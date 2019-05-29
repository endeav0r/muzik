use byteorder::{BigEndian, LittleEndian, WriteBytesExt};
use crate::SampleType;
use crate::constants::{BITS_PER_SAMPLE, NUM_CHANNELS, SAMPLE_RATE};

pub struct Wav {
    samples: Vec<SampleType>,
}

impl Wav {
    pub fn new(samples: Vec<SampleType>) -> Wav {
        Wav {
            samples: samples,
        }
    }

    pub fn encode(&self) -> Result<Vec<u8>, std::io::Error> {
        let mut buf: Vec<u8> = Vec::new();
        // http://soundfile.sapp.org/doc/WaveFormat/
        // Riff Header
        buf.write_u32::<BigEndian>(0x52494646)?;
        buf.write_u32::<LittleEndian>(4 + 24 + 8 + (self.samples().len() * 2) as u32)?;
        buf.write_u32::<BigEndian>(0x57415645)?;
        // Fmt Chunk
        // "fmt "
        buf.write_u32::<BigEndian>(0x666d7420)?;
        // Subchunk1Size, 16 for PCM
        buf.write_u32::<LittleEndian>(16)?;
        // AudioFormat, 1=PCM
        buf.write_u16::<LittleEndian>(1)?;
        // NumChannels, 2=Stereo
        buf.write_u16::<LittleEndian>(NUM_CHANNELS as u16)?;
        // SampleRate
        buf.write_u32::<LittleEndian>(SAMPLE_RATE as u32)?;
        // ByteRate
        buf.write_u32::<LittleEndian>((SAMPLE_RATE + NUM_CHANNELS + BITS_PER_SAMPLE / 8) as u32)?;
        // BlockAlign
        buf.write_u16::<LittleEndian>((NUM_CHANNELS + BITS_PER_SAMPLE / 8) as u16)?;
        // BitsPerSample
        buf.write_u16::<LittleEndian>(BITS_PER_SAMPLE as u16)?;
        // Data Chunk
        buf.write_u32::<BigEndian>(0x64617461)?;
        // SubChunk2Size
        buf.write_u32::<LittleEndian>((self.samples().len() * BITS_PER_SAMPLE / 8) as u32)?;
        for sample in self.samples() {
            buf.write_i16::<LittleEndian>(*sample)?;
        }

        Ok(buf)
    }

    pub fn samples(&self) -> &[SampleType] {
        &self.samples
    }
}
