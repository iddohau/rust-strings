use clap::Parser;
use rust_strings::{strings, Encoding, FileConfig, StdinConfig};
use std::path::Path;
use std::process::exit;
use std::str::FromStr;

#[derive(Parser, Debug)]
#[clap(version = "1.0", author = "Iddo Hauschner", name = "rust-strings")]
struct Opts {
    /// file path to run strings on, use "-" for stdin
    #[clap(name = "FILE_PATH_ARG")]
    file_path_arg: Option<String>,
    /// file path to run strings on, use "-" for stdin
    #[clap(short, long, name = "FILE_PATH")]
    file_path_flag: Option<String>,
    /// min length of string
    #[clap(short, long, default_value = "3")]
    min_length: usize,
    /// encoding of string
    #[clap(short, long, default_value = "ascii")]
    encoding: String,
    #[clap(short, long)]
    offset: bool,
}

fn get_file_path(options: &Opts) -> String {
    if matches!(options.file_path_arg, Some(_)) && matches!(options.file_path_flag, Some(_)) {
        eprintln!("You can't specify file path as argument and as flag together");
        exit(1);
    }
    let mut file_path = String::new();
    if let Some(file_path_arg) = &options.file_path_arg {
        file_path = file_path_arg.clone()
    }
    if let Some(file_path_flag) = &options.file_path_flag {
        file_path = file_path_flag.clone()
    }
    file_path
}

fn main() {
    let options = Opts::parse();
    let encoding = match Encoding::from_str(&options.encoding) {
        Ok(encoding) => encoding,
        Err(err) => {
            eprintln!("{}", err);
            exit(1);
        }
    };
    let file_path = get_file_path(&options);
    let extracted_strings = match file_path == "-" {
        true => strings(
            &StdinConfig::new()
                .with_min_length(options.min_length)
                .with_encoding(encoding),
        ),
        false => {
            let path: &Path = Path::new(&file_path);
            if !path.is_file() {
                eprintln!("File does not exists!");
                exit(1);
            }
            strings(
                &FileConfig::new(path)
                    .with_min_length(options.min_length)
                    .with_encoding(encoding),
            )
        }
    }
    .expect("Something went wrong!");
    for (string, offset) in extracted_strings {
        if options.offset {
            println!("{:10}: {}", offset, string);
        } else {
            println!("{}", string);
        }
    }
}
