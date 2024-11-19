use crate::domain::audio::AudioProcessor;

use hound::{SampleFormat, WavReader, WavSpec, WavWriter};
#[derive(Clone)]
pub struct WavHandler {
    pub pcm_data: Vec<i16>,
    pub sample_rate: u32,
    pub channels: u16,
    pub bits_per_sample: u16,
}

impl AudioProcessor for WavHandler {
    fn decode(&mut self, input_path: &str) -> Result<(), Box<dyn std::error::Error>> {
        let mut file = WavReader::open(input_path)?;

        self.sample_rate = file.spec().sample_rate;
        self.channels = file.spec().channels;
        self.bits_per_sample = file.spec().bits_per_sample;

        self.pcm_data = match file.spec().sample_format {
            SampleFormat::Int => file.samples::<i16>().map(|s| s.unwrap_or(0)).collect(),
            SampleFormat::Float => file
                .samples::<f32>()
                .map(|s| {
                    let sample = s.unwrap_or(0.0);
                    (sample * i16::MAX as f32) as i16
                })
                .collect(),
        };

        Ok(())
    }

    fn encode(&self, output_path: &str) -> Result<(), Box<dyn std::error::Error>> {
        let spec = WavSpec {
            channels: self.channels,
            sample_rate: self.sample_rate,
            bits_per_sample: self.bits_per_sample,
            sample_format: SampleFormat::Int,
        };

        let mut file = WavWriter::create(output_path, spec)?;

        for sample in &self.pcm_data {
            file.write_sample(*sample)?;
        }

        file.finalize()?;
        Ok(())
    }

    fn reverse(&mut self) {
        self.pcm_data.reverse();
    }

    fn clone_box(&self) -> Box<dyn AudioProcessor> {
        Box::new(self.clone())
    }
}

impl WavHandler {
    pub fn new() -> WavHandler {
        WavHandler {
            channels: 0,
            pcm_data: vec![],
            bits_per_sample: 0,
            sample_rate: 0,
        }
    }
}
