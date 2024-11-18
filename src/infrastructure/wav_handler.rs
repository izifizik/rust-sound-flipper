use crate::domain::audio::{AudioProcessor, AudioProperties};

use hound::{SampleFormat, WavReader, WavSpec, WavWriter};

pub struct WavHandler;

impl AudioProcessor for WavHandler {
    fn decode(
        &self,
        input_path: &str,
    ) -> Result<crate::domain::audio::AudioProperties, Box<dyn std::error::Error>> {
        let mut file = WavReader::open(input_path)?;

        let sample_rate = file.spec().sample_rate;
        let channels = file.spec().channels;
        let bits_per_sample = file.spec().bits_per_sample;

        let pcm_data: Vec<i16> = match file.spec().sample_format {
            SampleFormat::Int => file.samples::<i16>().map(|s| s.unwrap_or(0)).collect(),
            SampleFormat::Float => file
                .samples::<f32>()
                .map(|s| {
                    let sample = s.unwrap_or(0.0);
                    (sample * i16::MAX as f32) as i16
                })
                .collect(),
        };

        Ok(AudioProperties {
            pcm_data,
            bitrate: None,
            bits_per_sample: Some(bits_per_sample),
            channels,
            sample_rate,
        })
    }

    fn encode(
        &self,
        output_path: &str,
        prop: &crate::domain::audio::AudioProperties,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let spec = WavSpec {
            channels: prop.channels,
            sample_rate: prop.sample_rate,
            bits_per_sample: prop.bits_per_sample.unwrap(),
            sample_format: SampleFormat::Int,
        };

        let mut file = WavWriter::create(output_path, spec)?;

        for sample in &prop.pcm_data {
            file.write_sample(*sample)?;
        }

        file.finalize()?;
        Ok(())
    }
}
