use crate::infrastructure::mp3_handler::Mp3Handler;
use crate::infrastructure::wav_handler::WavHandler;
use crate::usecases::reverce_audio::ReverceAudioUseCase;
use crate::utils::formats::{Formatter, SupportedFormats};
use crate::utils::paths::Pather;
use core::str;

pub fn run(input: &str) {
    let mut formats = Formatter::new();
    if !formats.is_supported(input) {
        eprintln!("Error: file ext is not supported");
        return;
    }

    let out = match Pather::generate_output_path(input) {
        Ok(path) => path,
        Err(err) => {
            eprintln!("Error generate output path: {}", err);
            return;
        }
    };
    println!("{:?}", formats.ext);
    match formats.ext {
        Some(SupportedFormats::Mp3) => {
            let mp3handler = Mp3Handler;
            let use_case = ReverceAudioUseCase::new(mp3handler);
            match use_case.reverce_audio(input, &out) {
                Ok(_) => println!("Done. filename is {}", out),
                Err(err) => eprintln!("Error: {}", err),
            }
        }
        Some(SupportedFormats::Wav) => {
            let wav_handler = WavHandler;
            let use_case = ReverceAudioUseCase::new(wav_handler);
            match use_case.reverce_audio(input, &out) {
                Ok(_) => println!("Done. filename is {}", out),
                Err(err) => eprintln!("Error: {}", err),
            }
        }
        Some(SupportedFormats::Unsupported) => eprintln!("Error: unsupported type"),
        None => eprintln!("Error: where u're ext?"),
    }
}
