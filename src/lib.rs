//! # Rust Strings
//!
//! `rust-strings` is a library to extract ascii strings from binary data.
//! It is similar to the command `strings`.
//!
//! ## Examples:
//! ```
//! use std::path::PathBuf;
//! use rust_strings::{FileConfig, BytesConfig, strings, dump_strings, Encoding};
//!
//! let config = FileConfig::new("/bin/ls").with_min_length(5);
//! let extracted_strings = strings(&config);
//!
//! // Extract utf16le strings
//! let config = FileConfig::new("C:\\Windows\\notepad.exe")
//!     .with_min_length(15)
//!     .with_encoding(Encoding::UTF16LE);
//! let extracted_strings = strings(&config);
//!
//! // Extract ascii and utf16le strings
//! let config = FileConfig::new("C:\\Windows\\notepad.exe")
//!     .with_min_length(15)
//!     .with_encoding(Encoding::ASCII)
//!     .with_encoding(Encoding::UTF16LE);
//! let extracted_strings = strings(&config);
//!
//! let config = BytesConfig::new(b"test\x00".to_vec());
//! let extracted_strings = strings(&config);
//! assert_eq!(vec![(String::from("test"), 0)], extracted_strings.unwrap());
//!
//! // Dump strings into `strings.json` file.
//! let config = BytesConfig::new(b"test\x00".to_vec());
//! dump_strings(&config, PathBuf::from("strings.json"));
//! ```

use std::error::Error;

mod encodings;
mod strings;
mod strings_extractor;
mod strings_writer;

type ErrorResult = Result<(), Box<dyn Error>>;

pub use encodings::{Encoding, EncodingNotFoundError};
pub use strings::{dump_strings, strings, BytesConfig, Config, FileConfig};

#[cfg(feature = "python_bindings")]
mod python_bindings;
