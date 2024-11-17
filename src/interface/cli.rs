use crate::infrastructure::mp3_handler::Mp3Handler;
use crate::usecases::reverce_audio::ReverceAudioUseCase;
use core::str;
use std::path::Path;

pub fn run(input: &str) {
    let out = match generate_output_name(input) {
        Ok(path) => path,
        Err(err) => {
            eprintln!("Error generate output path: {}", err);
            return;
        }
    };

    let handler = Mp3Handler;
    let use_case = ReverceAudioUseCase::new(handler);

    println!("Start reverse...");
    match use_case.reverce_audio(input, &out) {
        Ok(_) => println!("Done. filename is {}", out),
        Err(err) => eprintln!("Error: {}", err),
    }
}

fn generate_output_name(input_path: &str) -> Result<String, Box<dyn std::error::Error>> {
    let path = Path::new(input_path);
    let parent = path.parent().unwrap_or_else(|| Path::new(""));
    let file_stem = path.file_stem().ok_or("invalid input filename");
    let extention = path.extension().ok_or("no ext found");

    let reversed_file_stem: String = file_stem?.to_string_lossy().chars().rev().collect();

    let out_filename = format!("{}.{}", reversed_file_stem, extention?.to_string_lossy(),);

    let out = parent.join(out_filename);
    Ok(out.to_string_lossy().into_owned())
}
