use std::collections::HashMap;
use std::path::Path;

use crate::domain::audio::AudioProcessor;
use crate::infrastructure::mp3_handler::Mp3Handler;
use crate::infrastructure::wav_handler::WavHandler;

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub enum SupportedFormats {
    Mp3,
    Wav,
    Unsupported,
}
pub struct Formatter {
    handlers: HashMap<SupportedFormats, Box<dyn AudioProcessor>>,
}

impl Formatter {
    pub fn new() -> Self {
        let mut handler: HashMap<SupportedFormats, Box<dyn AudioProcessor>> = HashMap::new();

        handler.insert(SupportedFormats::Mp3, Box::new(Mp3Handler::new()));
        handler.insert(SupportedFormats::Wav, Box::new(WavHandler::new()));

        Formatter { handlers: handler }
    }

    pub fn is_supported(&self, path: &str) -> Option<SupportedFormats> {
        Path::new(path)
            .extension()
            .and_then(|x| x.to_str())
            .map(|x| self.mapper(x))
    }

    pub fn get_handler(&mut self, format: SupportedFormats) -> Option<Box<dyn AudioProcessor>> {
        if let Some(handler) = self.handlers.remove(&format) {
            let h_clone = handler.clone_box();

            self.handlers.insert(format, handler);

            Some(h_clone)
        } else {
            None
        }
    }

    fn mapper(&self, ext: &str) -> SupportedFormats {
        match ext {
            "mp3" => SupportedFormats::Mp3,
            "wav" => SupportedFormats::Wav,
            _ => SupportedFormats::Unsupported,
        }
    }
}
