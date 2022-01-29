use std::error::Error;
use std::fs::File;
use std::io::{BufReader, Read};
use std::iter::Iterator;

use crate::encodings::Encoding;
use crate::strings_extractor::{new_strings_extractor, StringsExtractor};

const DEFAULT_MIN_LENGTH: usize = 3;
const DEFAULT_ENCODINGS: [Encoding; 1] = [Encoding::ASCII];

pub trait Config {
    #[doc(hidden)]
    fn consume<F>(&self, func: F) -> Option<Box<dyn Error>>
    where
        F: FnMut(usize, u8);
    #[doc(hidden)]
    fn get_min_length(&self) -> usize;
    #[doc(hidden)]
    fn get_encodings(&self) -> Vec<Encoding>;
}

macro_rules! impl_config {
    () => {
        fn get_min_length(&self) -> usize {
            self.min_length
        }
        fn get_encodings(&self) -> Vec<Encoding> {
            if self.encodings.is_empty() {
                return DEFAULT_ENCODINGS.to_vec();
            }
            self.encodings.clone()
        }
    };
}

macro_rules! impl_default {
    () => {
        pub fn with_min_length(mut self, min_length: usize) -> Self {
            self.min_length = min_length;
            self
        }

        pub fn with_encoding(mut self, encoding: Encoding) -> Self {
            self.encodings.push(encoding);
            self
        }

        pub fn with_encodings(mut self, encodings: Vec<Encoding>) -> Self {
            self.encodings = encodings;
            self
        }
    };
}

pub struct FileConfig<'a> {
    pub file_path: &'a str,
    pub min_length: usize,
    pub encodings: Vec<Encoding>,
    pub buffer_size: usize,
}

impl<'a> FileConfig<'a> {
    const DEFAULT_BUFFER_SIZE: usize = 1024 * 1024;

    pub fn new(file_path: &'a str) -> Self {
        FileConfig {
            file_path,
            min_length: DEFAULT_MIN_LENGTH,
            encodings: vec![],
            buffer_size: FileConfig::DEFAULT_BUFFER_SIZE,
        }
    }

    pub fn with_buffer_size(mut self, buffer_size: usize) -> Self {
        self.buffer_size = buffer_size;
        self
    }

    impl_default!();
}

impl<'a> Config for FileConfig<'a> {
    fn consume<F>(&self, mut func: F) -> Option<Box<dyn Error>>
    where
        F: FnMut(usize, u8),
    {
        let file_result = File::open(&self.file_path);
        if let Err(err) = file_result {
            return Some(Box::new(err));
        }
        let file = file_result.unwrap();
        let buf_reader = BufReader::with_capacity(self.buffer_size, file);
        buf_reader
            .bytes()
            .enumerate()
            .for_each(|(i, b)| func(i, b.unwrap()));
        None
    }

    impl_config!();
}

pub struct BytesConfig {
    pub bytes: Vec<u8>,
    pub min_length: usize,
    pub encodings: Vec<Encoding>,
}

impl BytesConfig {
    pub fn new(bytes: Vec<u8>) -> Self {
        BytesConfig {
            bytes,
            min_length: DEFAULT_MIN_LENGTH,
            encodings: vec![],
        }
    }

    impl_default!();
}

impl Config for BytesConfig {
    fn consume<F>(&self, mut func: F) -> Option<Box<dyn Error>>
    where
        F: FnMut(usize, u8),
    {
        self.bytes.iter().enumerate().for_each(|(i, b)| func(i, *b));
        None
    }

    impl_config!();
}

/// Extract strings from binary data.
///
/// Examples:
/// ```
/// use rust_strings::{FileConfig, BytesConfig, strings, Encoding};
///
/// let config = FileConfig::new("/bin/ls").with_min_length(5);
/// let extracted_strings = strings(&config);
/// 
/// // Extract utf16le strings
/// let config = FileConfig::new("C:\\Windows\\notepad.exe")
///     .with_min_length(15)
///     .with_encoding(Encoding::UTF16LE);
/// let extracted_strings = strings(&config);
/// 
/// // Extract ascii and utf16le strings
/// let config = FileConfig::new("C:\\Windows\\notepad.exe")
///     .with_min_length(15)
///     .with_encoding(Encoding::ASCII)
///     .with_encoding(Encoding::UTF16LE);
/// let extracted_strings = strings(&config);
///
/// let config = BytesConfig::new(vec![116, 101, 115, 116, 0, 0]);
/// let extracted_strings = strings(&config);
/// assert_eq!(vec![(String::from("test"), 0)], extracted_strings.unwrap());
/// ```
pub fn strings<T: Config>(strings_config: &T) -> Result<Vec<(String, u64)>, Box<dyn Error>> {
    let mut strings_vector: Vec<(String, u64)> = Vec::new();
    let min_length = strings_config.get_min_length();
    let mut strings_extractors: Vec<Box<dyn StringsExtractor>> = strings_config
        .get_encodings()
        .iter()
        .map(|e| new_strings_extractor(*e, min_length))
        .collect();
    let err = strings_config.consume(|offset: usize, c: u8| {
        strings_extractors.iter_mut().for_each(|strings_extractor| {
            if strings_extractor.can_consume(c) {
                strings_extractor.consume(offset as u64, c);
            } else if let Some((offset, string)) = strings_extractor.get_string() {
                strings_vector.push((string, offset));
            }
        });
    });
    if let Some(err) = err {
        return Err(err);
    }
    strings_extractors.iter_mut().for_each(|strings_extractor| {
        if let Some((offset, string)) = strings_extractor.get_string() {
            strings_vector.push((string, offset));
        }
    });
    Ok(strings_vector)
}
