use crate::domain::audio::AudioProcessor;

pub struct ReverceAudioUseCase {
    processor: Box<dyn AudioProcessor>,
}

impl ReverceAudioUseCase {
    pub fn new(processor: Box<dyn AudioProcessor>) -> Self {
        Self { processor }
    }

    pub fn reverse_audio(
        &mut self,
        input_path: &str,
        output_path: &str,
    ) -> Result<(), Box<dyn std::error::Error>> {
        self.processor.decode(input_path)?;
        self.processor.reverse();
        self.processor.encode(output_path)?;

        Ok(())
    }
}
