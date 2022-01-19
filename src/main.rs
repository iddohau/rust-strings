use clap::Parser;
use rust_strings::{strings, Encoding, FileConfig};
use std::path::{Path, PathBuf};
use std::process::exit;
use std::str::FromStr;

#[derive(Parser, Debug)]
#[clap(version = "1.0", author = "Iddo Hauschner", name = "rust-strings")]
struct Opts {
    /// file path to run strings on
    #[clap(short, long)]
    file_path: PathBuf,
    /// min length of string
    #[clap(short, long, default_value = "3")]
    min_length: usize,
    /// encoding of string
    #[clap(short, long, default_value = "ascii")]
    encoding: String,
}

fn main() {
    let options = Opts::parse();
    let file_path = options.file_path;
    let path: &Path = file_path.as_path();
    if !path.is_file() {
        eprintln!("File does not exists!");
        exit(1);
    }
    let encoding = match Encoding::from_str(&options.encoding) {
        Ok(encoding) => encoding,
        Err(err) => {
            eprintln!("{}", err);
            exit(1);
        }
    };
    let strings_config = FileConfig::new(file_path)
        .with_min_length(options.min_length)
        .with_encoding(encoding);
    let strings_vector = strings(&strings_config).unwrap();
    for (string, offset) in strings_vector {
        println!("{}: {}", offset, string);
    }
}
