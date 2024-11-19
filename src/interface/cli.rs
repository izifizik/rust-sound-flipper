use crate::usecases::reverce_audio::ReverceAudioUseCase;
use crate::utils::formats::Formatter;
use crate::utils::paths::Pather;
use core::str;

pub fn run(input: &str) {
    let out = match Pather::generate_output_path(input) {
        Ok(path) => path,
        Err(err) => {
            eprintln!("Error generate output path: {}", err);
            return;
        }
    };

    let mut formats = Formatter::new();
    match formats.is_supported(input) {
        Some(format) => match formats.get_handler(format) {
            Some(hanlder) => {
                let mut use_case = ReverceAudioUseCase::new(hanlder);
                if let Err(err) = use_case.reverse_audio(input, &out) {
                    eprintln!("Error reverse audio with error: {}", err);
                    return;
                }
            }
            None => {
                eprintln!("Error: handler not found for format: {:?}", format);
                return;
            }
        },
        None => {
            eprintln!("Error: unsuported format");
            return;
        }
    }
}
