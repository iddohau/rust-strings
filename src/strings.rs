use std::error::Error;
use std::fs::File;
use std::io::{BufReader, Read};
use std::iter::Iterator;

use crate::encodings::Encoding;

const DEFAULT_MIN_LENGTH: usize = 3;
const DEFAULT_ENCODING: Encoding = Encoding::ASCII;

pub trait Config {
    #[doc(hidden)]
    fn consume<F>(&self, func: F) -> Option<Box<dyn Error>>
    where
        F: FnMut(usize, u8);
    #[doc(hidden)]
    fn get_min_length(&self) -> usize;
}

pub struct FileConfig<'a> {
    pub file_path: &'a str,
    pub min_length: usize,
    pub encoding: Encoding,
    pub buffer_size: usize,
}

impl <'a>FileConfig<'a> {
    const DEFAULT_BUFFER_SIZE: usize = 1024 * 1024;

    pub fn new(file_path: &'a str) -> Self {
        FileConfig {
            file_path,
            min_length: DEFAULT_MIN_LENGTH,
            encoding: DEFAULT_ENCODING,
            buffer_size: FileConfig::DEFAULT_BUFFER_SIZE,
        }
    }

    pub fn with_min_length(mut self, min_length: usize) -> Self {
        self.min_length = min_length;
        self
    }

    pub fn with_buffer_size(mut self, buffer_size: usize) -> Self {
        self.buffer_size = buffer_size;
        self
    }

    pub fn with_encoding(mut self, encoding: Encoding) -> Self {
        self.encoding = encoding;
        self
    }
}

impl <'a>Config for FileConfig<'a> {
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

    fn get_min_length(&self) -> usize {
        self.min_length
    }
}

pub struct BytesConfig {
    pub bytes: Vec<u8>,
    pub min_length: usize,
    pub encoding: Encoding,
}

impl BytesConfig {
    pub fn new(bytes: Vec<u8>) -> Self {
        BytesConfig {
            bytes,
            min_length: DEFAULT_MIN_LENGTH,
            encoding: DEFAULT_ENCODING,
        }
    }

    pub fn with_min_length(mut self, min_length: usize) -> Self {
        self.min_length = min_length;
        self
    }

    pub fn with_encoding(mut self, encoding: Encoding) -> Self {
        self.encoding = encoding;
        self
    }
}

impl Config for BytesConfig {
    fn consume<F>(&self, mut func: F) -> Option<Box<dyn Error>>
    where
        F: FnMut(usize, u8),
    {
        self.bytes.iter().enumerate().for_each(|(i, b)| func(i, *b));
        None
    }

    fn get_min_length(&self) -> usize {
        self.min_length
    }
}

/// Extract strings from binary data.
/// 
/// Examples:
/// ```
/// let config = FileConfig::new("/bin/ls").with_min_length(5);
/// let extracted_strings = strings(&config);
/// 
/// let config = BytesConfig::new(vec![116, 101, 115, 116, 0, 0]);
/// let extracted_strings = strings(&config);
/// // [("test", 0)]
/// ```
pub fn strings<T: Config>(
    strings_config: &T,
) -> Result<Vec<(String, u64)>, Box<dyn Error>> {
    let mut string = String::with_capacity(strings_config.get_min_length());
    let mut current_offset: Option<u64> = None;
    let mut strings_vector: Vec<(String, u64)> = Vec::new();
    let min_length = strings_config.get_min_length();
    let err = strings_config.consume(|offset: usize, c: u8| {
        if is_printable_character(c) {
            if current_offset == None {
                current_offset = Some(offset as u64);
            }
            string.push(c as char);
        } else {
            if let Some(value) = current_offset {
                add_string_to_strings_list(&mut string, value, &mut strings_vector, min_length);
            }
            string.clear();
            current_offset = None;
        }
    });
    if let Some(err) = err {
        return Err(err);
    }
    if let Some(value) = current_offset {
        add_string_to_strings_list(&mut string, value, &mut strings_vector, min_length);
    }
    Ok(strings_vector)
}

fn add_string_to_strings_list(
    string: &mut String,
    offset: u64,
    strings_vector: &mut Vec<(String, u64)>,
    min_length: usize,
) {
    if string.len() >= min_length {
        strings_vector.push((string.to_string(), offset));
    }
}

fn is_printable_character(c: u8) -> bool {
    (32..=126).contains(&c) || (9..=13).contains(&c)
}
