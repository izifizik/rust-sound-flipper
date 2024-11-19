use crate::domain::audio::AudioProcessor;

use lame::Lame;
use symphonia::core::audio::{AudioBufferRef, Signal};
use symphonia::core::codecs::DecoderOptions;
use symphonia::core::formats::FormatOptions;
use symphonia::core::io::MediaSourceStream;
use symphonia::core::meta::MetadataOptions;
use symphonia::core::probe::Hint;
use symphonia::default::{get_codecs, get_probe};

use std::fs::File;
use std::i16;
use std::io::{BufWriter, Write};
#[derive(Clone)]
pub struct Mp3Handler {
    pub pcm_data: Vec<i16>,
    pub sample_rate: u32,
    pub channels: u16,
    pub bitrate: i32,
}

impl AudioProcessor for Mp3Handler {
    fn decode(&mut self, input_path: &str) -> Result<(), Box<dyn std::error::Error>> {
        let file_size = std::fs::metadata(input_path)?.len();
        let (mut format, track) = Self::init_decoder(input_path)?;

        let duration = Self::calculate_duration(&track)?;
        self.bitrate = Self::calculate_bitrate(file_size, duration);

        (self.pcm_data, self.sample_rate, self.channels) = Self::extract_pcm(&mut format, &track)?;

        Ok(())
    }

    fn encode(&self, output_path: &str) -> Result<(), Box<dyn std::error::Error>> {
        let mut lame = Self::init_lame(self)?;
        Self::write_mp3(&mut lame, output_path, &self.pcm_data)?;
        Ok(())
    }

    fn reverse(&mut self) {
        self.pcm_data.reverse();
    }

    fn clone_box(&self) -> Box<dyn AudioProcessor> {
        Box::new(self.clone())
    }
}

impl Mp3Handler {
    pub fn new() -> Mp3Handler {
        Mp3Handler {
            pcm_data: vec![],
            bitrate: 0,
            channels: 0,
            sample_rate: 0,
        }
    }

    fn init_decoder(
        input_path: &str,
    ) -> Result<
        (
            Box<dyn symphonia::core::formats::FormatReader>,
            symphonia::core::formats::Track,
        ),
        Box<dyn std::error::Error>,
    > {
        let file = File::open(input_path)?;
        let mss = MediaSourceStream::new(Box::new(file), Default::default());
        let mut hint = Hint::new();
        hint.with_extension("mp3");

        let probed = get_probe().format(
            &hint,
            mss,
            &FormatOptions::default(),
            &MetadataOptions::default(),
        )?;
        let format = probed.format;
        let track = format.default_track().cloned().ok_or("No track found")?;
        Ok((format, track))
    }

    fn calculate_duration(
        track: &symphonia::core::formats::Track,
    ) -> Result<f64, Box<dyn std::error::Error>> {
        let time_base = track.codec_params.time_base.ok_or("Missing time base")?;
        let n_frames = track.codec_params.n_frames.ok_or("Missing frame count")?;
        Ok(n_frames as f64 * time_base.numer as f64 / time_base.denom as f64)
    }

    fn calculate_bitrate(file_size: u64, duration: f64) -> i32 {
        (file_size as f64 * 8.0 / duration / 1000.0) as i32
    }

    fn extract_pcm(
        format: &mut Box<dyn symphonia::core::formats::FormatReader>,
        track: &symphonia::core::formats::Track,
    ) -> Result<(Vec<i16>, u32, u16), Box<dyn std::error::Error>> {
        let mut decoder = get_codecs().make(&track.codec_params, &DecoderOptions::default())?;
        let mut pcm_data = Vec::new();
        let mut sample_rate = 0;
        let mut channels = 0;

        while let Ok(packet) = format.next_packet() {
            if let Ok(decoded) = decoder.decode(&packet) {
                match decoded {
                    AudioBufferRef::S16(buffer) => {
                        sample_rate = buffer.spec().rate;
                        channels = buffer.spec().channels.count() as u16;
                        pcm_data.extend_from_slice(buffer.chan(0));
                    }
                    AudioBufferRef::F32(buffer) => {
                        sample_rate = buffer.spec().rate;
                        channels = buffer.spec().channels.count() as u16;
                        let slice: Vec<i16> = buffer
                            .chan(0)
                            .iter()
                            .map(|&x| (x * i16::MAX as f32) as i16)
                            .collect();
                        pcm_data.extend_from_slice(&slice);
                    }
                    _ => {
                        println!("Unsupported audio format.");
                    }
                }
            }
        }

        Ok((pcm_data, sample_rate, channels))
    }

    fn init_lame(&self) -> Result<Lame, Box<dyn std::error::Error>> {
        let mut lame = Lame::new().ok_or("Failed to initialize LAME")?;
        lame.set_sample_rate(self.sample_rate)
            .map_err(|e| format!("Error setting sample rate: {:?}", e))?;
        lame.set_channels(self.channels as u8)
            .map_err(|e| format!("Error setting channels: {:?}", e))?;
        lame.set_kilobitrate(self.bitrate)
            .map_err(|e| format!("Error setting bitrate: {:?}", e))?;
        lame.init_params()
            .map_err(|e| format!("Error initializing LAME parameters: {:?}", e))?;
        Ok(lame)
    }

    fn write_mp3(
        lame: &mut Lame,
        output_path: &str,
        pcm_data: &[i16],
    ) -> Result<(), Box<dyn std::error::Error>> {
        let file = File::create(output_path)?;
        let mut writer = BufWriter::new(file);

        let buffer_size = (1.25 * pcm_data.len() as f32 + 7200.0) as usize;
        let mut buffer = vec![0; buffer_size];

        let bytes_written = lame
            .encode(pcm_data, pcm_data, &mut buffer)
            .map_err(|e| format!("Error encoding MP3: {:?}", e))?;
        writer.write_all(&buffer[..bytes_written])?;

        println!("Encoding completed successfully!");
        Ok(())
    }
}
