use crate::domain::audio::AudioProcessor;

pub struct ReverceAudioUseCase<T: AudioProcessor> {
    processor: T,
}

impl<T: AudioProcessor> ReverceAudioUseCase<T> {
    pub fn new(processor: T) -> Self {
        Self { processor }
    }

    pub fn reverce_audio(
        &self,
        input_path: &str,
        output_path: &str,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let mut prop = self.processor.decode(input_path)?;
        prop.pcm_data.reverse();
        self.processor.encode(output_path, &prop)?;

        Ok(())
    }
}
