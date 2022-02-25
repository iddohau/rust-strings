use std::cell::RefCell;
use std::error::Error;
use std::fs::File;
use std::io::{BufReader, Read};
use std::iter::Iterator;
use std::path::PathBuf;
use std::rc::Rc;
use std::result::Result;

use crate::encodings::Encoding;
use crate::strings_extractor::{new_strings_extractor, StringsExtractor};
use crate::strings_writer::{JsonWriter, StringWriter, VectorWriter};
use crate::ErrorResult;

const DEFAULT_MIN_LENGTH: usize = 3;
const DEFAULT_ENCODINGS: [Encoding; 1] = [Encoding::ASCII];

pub trait Config {
    #[doc(hidden)]
    fn consume<F>(&self, func: F) -> ErrorResult
    where
        F: FnMut(usize, u8) -> ErrorResult;
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
    fn consume<F>(&self, mut func: F) -> ErrorResult
    where
        F: FnMut(usize, u8) -> ErrorResult,
    {
        let file_result = File::open(&self.file_path);
        if let Err(err) = file_result {
            return Err(Box::new(err));
        }
        let file = file_result.unwrap();
        let buf_reader = BufReader::with_capacity(self.buffer_size, file);
        buf_reader
            .bytes()
            .enumerate()
            .try_for_each(|(i, b)| func(i, b.unwrap()))?;
        Ok(())
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
    fn consume<F>(&self, mut func: F) -> ErrorResult
    where
        F: FnMut(usize, u8) -> ErrorResult,
    {
        self.bytes
            .iter()
            .enumerate()
            .try_for_each(|(i, b)| func(i, *b))?;
        Ok(())
    }

    impl_config!();
}

fn _strings<T: Config, W: StringWriter>(
    strings_config: &T,
    strings_writer: Rc<RefCell<W>>,
) -> ErrorResult {
    let min_length = strings_config.get_min_length();
    let mut strings_extractors: Vec<Box<dyn StringsExtractor>> = strings_config
        .get_encodings()
        .iter()
        .map(|e| new_strings_extractor(strings_writer.clone(), *e, min_length))
        .collect();
    strings_config.consume(|offset: usize, c: u8| {
        strings_extractors
            .iter_mut()
            .try_for_each(|strings_extractor| -> ErrorResult {
                if strings_extractor.can_consume(c) {
                    strings_extractor.consume(offset as u64, c)?;
                } else {
                    strings_extractor.stop_consume()?;
                }
                Ok(())
            })?;
        Ok(())
    })?;
    strings_extractors
        .iter_mut()
        .try_for_each(|strings_extractor| -> ErrorResult {
            strings_extractor.stop_consume()?;
            Ok(())
        })?;
    Ok(())
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
/// let config = BytesConfig::new(b"test\x00".to_vec());
/// let extracted_strings = strings(&config);
/// assert_eq!(vec![(String::from("test"), 0)], extracted_strings.unwrap());
/// ```
pub fn strings<T: Config>(strings_config: &T) -> Result<Vec<(String, u64)>, Box<dyn Error>> {
    let vector_writer = Rc::new(RefCell::new(VectorWriter::new()));
    _strings(strings_config, vector_writer.clone())?;
    let result = Ok(vector_writer.borrow_mut().get_strings());
    result
}

/// Dump strings from binary data to json file.
///
/// Examples:
/// ```
/// use std::path::PathBuf;
/// use rust_strings::{BytesConfig, dump_strings};
///
/// let config = BytesConfig::new(b"test\x00".to_vec());
/// dump_strings(&config, PathBuf::from("strings.json"));
///
pub fn dump_strings<T: Config>(strings_config: &T, output: PathBuf) -> ErrorResult {
    let output_file = File::create(output)?;
    let vector_writer = Rc::new(RefCell::new(JsonWriter::new(output_file)));
    _strings(strings_config, vector_writer.clone())?;
    vector_writer.borrow_mut().finish()?;
    Ok(())
}
