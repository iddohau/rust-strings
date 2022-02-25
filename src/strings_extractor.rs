use std::cell::RefCell;
use std::mem::take;
use std::rc::Rc;

use crate::encodings::Encoding;
use crate::strings_writer::StringWriter;
use crate::ErrorResult;

pub trait StringsExtractor {
    fn can_consume(&self, c: u8) -> bool;
    fn consume(&mut self, offset: u64, c: u8) -> ErrorResult;
    fn stop_consume(&mut self) -> ErrorResult;
}

pub struct AsciiExtractor<T> {
    writer: Rc<RefCell<T>>,
    min_length: usize,
    current_string: Vec<u8>,
    offset: u64,
    is_start_writing: bool,
}

pub struct Utf16Extractor<T> {
    writer: Rc<RefCell<T>>,
    is_big_endian: bool,
    is_last_char_null: Option<bool>,
    min_length: usize,
    current_string: Vec<u8>,
    offset: Option<u64>,
    is_start_writing: bool,
}

pub fn new_strings_extractor<'a, T>(
    writer: Rc<RefCell<T>>,
    encoding: Encoding,
    min_length: usize,
) -> Box<dyn StringsExtractor + 'a>
where
    T: StringWriter + 'a,
{
    match encoding {
        Encoding::ASCII => Box::new(AsciiExtractor {
            writer,
            min_length,
            current_string: Vec::with_capacity(min_length),
            offset: 0,
            is_start_writing: false,
        }),
        Encoding::UTF16LE => Box::new(Utf16Extractor {
            writer,
            is_big_endian: false,
            is_last_char_null: None,
            min_length,
            current_string: Vec::with_capacity(min_length),
            offset: None,
            is_start_writing: false,
        }),
        Encoding::UTF16BE => Box::new(Utf16Extractor {
            writer,
            is_big_endian: true,
            is_last_char_null: None,
            min_length,
            current_string: Vec::with_capacity(min_length),
            offset: None,
            is_start_writing: false,
        }),
    }
}

fn is_printable_character(c: u8) -> bool {
    (32..=126).contains(&c) || (9..=10).contains(&c) || c == 13
}

impl<T> StringsExtractor for AsciiExtractor<T>
where
    T: StringWriter,
{
    fn can_consume(&self, c: u8) -> bool {
        is_printable_character(c)
    }

    fn consume(&mut self, offset: u64, c: u8) -> ErrorResult {
        if self.is_start_writing {
            self.writer.borrow_mut().write_char(c as char)?;
        } else if self.current_string.is_empty() && !self.is_start_writing {
            self.offset = offset;
            self.current_string.push(c);
        } else if self.current_string.len() == self.min_length - 1 && !self.is_start_writing {
            self.is_start_writing = true;
            self.current_string.push(c);
            self.writer
                .borrow_mut()
                .start_string_consume(take(&mut self.current_string), self.offset)?;
        } else {
            self.current_string.push(c);
        }
        Ok(())
    }

    fn stop_consume(&mut self) -> ErrorResult {
        if self.is_start_writing {
            self.writer.borrow_mut().finish_string_consume()?;
        }
        self.is_start_writing = false;
        self.current_string.clear();
        Ok(())
    }
}

impl<T> StringsExtractor for Utf16Extractor<T>
where
    T: StringWriter,
{
    fn can_consume(&self, c: u8) -> bool {
        let is_char_null = c == 0;
        match self.is_last_char_null {
            None => {
                (self.is_big_endian && is_char_null)
                    || (!self.is_big_endian && is_printable_character(c))
            }
            Some(is_last_char_null) => {
                let is_char_printable = is_printable_character(c);
                (!is_last_char_null && is_char_null) || (is_last_char_null && is_char_printable)
            }
        }
    }

    fn consume(&mut self, offset: u64, c: u8) -> ErrorResult {
        let is_char_null = c == 0;
        self.is_last_char_null = Some(is_char_null);
        if is_char_null {
            // This is here because big endian is null first
            if self.current_string.is_empty() {
                self.offset = Some(offset);
            }
            return Ok(());
        }
        if self.is_start_writing {
            self.writer.borrow_mut().write_char(c as char)?;
        } else if self.current_string.is_empty() && !self.is_start_writing {
            if self.offset == None {
                self.offset = Some(offset);
            }
            self.current_string.push(c);
        } else if self.current_string.len() == self.min_length - 1 && !self.is_start_writing {
            self.is_start_writing = true;
            self.current_string.push(c);
            self.writer
                .borrow_mut()
                .start_string_consume(take(&mut self.current_string), self.offset.unwrap())?;
        } else {
            self.current_string.push(c);
        }
        Ok(())
    }

    fn stop_consume(&mut self) -> ErrorResult {
        if self.is_start_writing {
            self.writer.borrow_mut().finish_string_consume()?;
        }
        self.is_last_char_null = None;
        self.is_start_writing = false;
        self.offset = None;
        self.current_string.clear();
        Ok(())
    }
}
