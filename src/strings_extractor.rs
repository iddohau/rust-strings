use std::mem::take;

use crate::encodings::Encoding;

pub trait StringsExtractor {
    fn can_consume(&self, c: u8) -> bool;
    fn consume(&mut self, offset: u64, c: u8);
    fn get_string(&mut self) -> Option<(u64, String)>;
    fn encode(&self, string: Vec<u8>) -> String;
}

pub struct AsciiExtractor {
    current_string: Vec<u8>,
    offset: u64,
    min_length: usize,
}

pub struct Utf16Extractor {
    current_string: Vec<u8>,
    offset: u64,
    min_length: usize,
    before: bool,
}

pub fn new_strings_extractor(encoding: Encoding, min_length: usize) -> Box<dyn StringsExtractor> {
    match encoding {
        Encoding::ASCII => Box::new(AsciiExtractor {
            current_string: Vec::with_capacity(min_length),
            offset: 0,
            min_length,
        }),
        Encoding::UTF16LE => Box::new(Utf16Extractor {
            current_string: Vec::with_capacity(min_length * 2),
            offset: 0,
            min_length,
            before: false,
        }),
        Encoding::UTF16BE => Box::new(Utf16Extractor {
            current_string: Vec::with_capacity(min_length * 2),
            offset: 0,
            min_length,
            before: true,
        }),
    }
}

macro_rules! consume {
    () => {
        fn consume(&mut self, offset: u64, c: u8) {
            if self.current_string.is_empty() {
                self.offset = offset;
            }
            self.current_string.push(c);
        }
    };
}

macro_rules! get_string {
    () => {
        fn get_string(&mut self) -> Option<(u64, String)> {
            if self.current_string.len() > self.min_length {
                let current_string = take(&mut self.current_string);
                let string = self.encode(current_string);
                return Some((self.offset, string));
            }
            self.current_string.clear();
            None
        }
    };
}

fn is_printable_character(c: u8) -> bool {
    (32..=126).contains(&c) || (9..=13).contains(&c)
}

impl StringsExtractor for AsciiExtractor {
    fn can_consume(&self, c: u8) -> bool {
        is_printable_character(c)
    }

    fn encode(&self, string: Vec<u8>) -> String {
        String::from_utf8(string).unwrap()
    }

    consume!();
    get_string!();
}

impl StringsExtractor for Utf16Extractor {
    fn can_consume(&self, c: u8) -> bool {
        if self.current_string.is_empty() {
            return (self.before && c == 0) || (!self.before && is_printable_character(c));
        }
        let is_last_char_null = self.current_string[self.current_string.len() - 1] == 0;
        let is_char_null = c == 0;
        let is_char_printable = is_printable_character(c);
        (!is_last_char_null && is_char_null) || (is_last_char_null && is_char_printable)
    }

    fn encode(&self, string: Vec<u8>) -> String {
        let mut new_string = String::with_capacity(string.len() / 2);
        let start = match self.before {
            true => 1,
            false => 0,
        };
        for c in string.into_iter().skip(start).step_by(2) {
            new_string.push(c as char);
        }
        new_string
    }

    consume!();
    get_string!();
}
