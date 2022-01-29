//! # Rust Strings
//! 
//! `rust_strings` is a library to extract ascii strings from binary data.
//! It is similar to the command `strings`.

mod strings;
mod encodings;
mod strings_extractor;

pub use strings::{BytesConfig, Config, FileConfig, strings};
pub use encodings::{Encoding, EncodingNotFoundError};

#[cfg(feature = "python_bindings")]
mod python_bindings;
