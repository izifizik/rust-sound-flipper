pub trait AudioProcessor {
    fn decode(
        &self,
        input_path: &str,
    ) -> Result<AudioProperties, Box<dyn std::error::Error>>;
    fn encode(
        &self,
        output_path: &str,
        prop: &AudioProperties,
    ) -> Result<(), Box<dyn std::error::Error>>;
}

#[derive(Debug)]
pub struct AudioProperties {
    pub pcm_data: Vec<i16>,
    pub sample_rate: u32,
    pub channels: u16,
    pub bitrate: i32,
}