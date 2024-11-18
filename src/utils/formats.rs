use std::collections::HashSet;
use std::path::Path;

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub enum SupportedFormats {
    Mp3,
    Wav,
    Unsupported,
}
pub struct Formatter {
    format: HashSet<SupportedFormats>,
    pub ext: Option<SupportedFormats>,
}

impl Formatter {
    pub fn new() -> Self {
        let format = HashSet::from([SupportedFormats::Mp3, SupportedFormats::Wav]);
        Formatter {
            format: (format),
            ext: None,
        }
    }

    pub fn is_supported(&mut self, path: &str) -> bool {
        Path::new(path)
            .extension()
            .and_then(|x| x.to_str())
            .map_or(false, |x| {
                let ext = &self.mapper(&x.to_ascii_lowercase()[..]);
                self.ext = Some(*ext);
                self.format.contains(ext)
            })
    }

    fn mapper(&self, ext: &str) -> SupportedFormats {
        match ext {
            "mp3" => SupportedFormats::Mp3,
            "wav" => SupportedFormats::Wav,
            _ => SupportedFormats::Unsupported,
        }
    }
}
