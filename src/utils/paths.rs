use std::path::Path;

pub struct Pather;

impl Pather {
    pub fn generate_output_path(input_path: &str) -> Result<String, Box<dyn std::error::Error>> {
        let path = Path::new(input_path);
        let parent = path.parent().unwrap_or_else(|| Path::new(""));
        let file_stem = path.file_stem().ok_or("invalid input filename");
        let extention = path.extension().ok_or("no ext found");

        let reversed_file_stem: String = file_stem?.to_string_lossy().chars().rev().collect();

        let out_filename = format!("{}.{}", reversed_file_stem, extention?.to_string_lossy(),);

        let out = parent.join(out_filename);

        Ok(out.to_string_lossy().into_owned())
    }
}
