use anyhow::Result;
use clap::{Parser, ValueEnum};
use flate2::write::{GzDecoder, GzEncoder};
use flate2::Compression;
use std::fs::File;
use std::io::{Read, Write};

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
enum CompressAction {
    Encode,
    Decode,
}

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Encode or Decode
    #[arg(value_enum)]
    action: CompressAction,

    /// File to compress.
    #[arg(short, long)]
    input_file: String,

    /// File to output into.
    #[arg(short, long)]
    output_file: String,
}

fn gzip(input_path: &str, output_path: &str, action: CompressAction) -> Result<()> {
    let mut input_file = File::open(input_path).expect("unable to read input file");
    let output_file = File::create(output_path).expect("unable to create new gzip file");

    let mut buffer = Vec::new();
    let _ = input_file.read_to_end(&mut buffer);

    match action {
        CompressAction::Encode => {
            let mut encoder = GzEncoder::new(output_file, Compression::default());
            encoder.write_all(&buffer)?;
            encoder.finish()?;
        }
        CompressAction::Decode => {
            let mut decoder = GzDecoder::new(output_file);
            decoder.write_all(&buffer)?;
            decoder.finish()?;
        }
    }

    return Ok(());
}

fn main() -> Result<()> {
    let args = Args::parse();

    if let Err(e) = gzip(&args.input_file, &args.output_file, args.action) {
        eprintln!("Gzip failed: {}", e);
    }

    return Ok(());
}
