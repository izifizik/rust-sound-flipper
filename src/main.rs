use clap::Parser;

mod domain;
mod infrastructure;
mod interface;
mod usecases;

#[derive(Parser, Debug)]
#[command(name = "Rust Sound Flipper")]
#[command(about = "A tool to reverse audio files", long_about = None)]
struct Args {
    #[arg(short, long)]
    input: String,
}
fn main() {
    let args = Args::parse();

    interface::cli::run(&args.input);
}
