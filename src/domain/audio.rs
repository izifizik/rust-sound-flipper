pub trait AudioProcessor {
    fn decode(&mut self, input_path: &str) -> Result<(), Box<dyn std::error::Error>>;
    fn encode(&self, output_path: &str) -> Result<(), Box<dyn std::error::Error>>;
    fn reverse(&mut self);

    fn clone_box(&self) -> Box<dyn AudioProcessor>;
}
