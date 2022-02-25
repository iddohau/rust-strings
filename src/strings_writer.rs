use std::io::Write;
use std::mem::take;

use crate::ErrorResult;

pub trait StringWriter {
    fn start_string_consume(&mut self, string: Vec<u8>, offset: u64) -> ErrorResult;
    fn write_char(&mut self, c: char) -> ErrorResult;
    fn finish_string_consume(&mut self) -> ErrorResult;
}

pub struct VectorWriter {
    vec: Vec<(String, u64)>,
    current_string: String,
    current_offset: u64,
}

impl VectorWriter {
    pub fn new() -> Self {
        VectorWriter {
            vec: vec![],
            current_offset: 0,
            current_string: String::new(),
        }
    }
}

impl StringWriter for VectorWriter {
    fn start_string_consume(&mut self, string: Vec<u8>, offset: u64) -> ErrorResult {
        self.current_offset = offset;
        self.current_string = String::with_capacity(string.len());
        string
            .into_iter()
            .for_each(|c| self.current_string.push(c as char));
        Ok(())
    }

    fn write_char(&mut self, c: char) -> ErrorResult {
        self.current_string.push(c);
        Ok(())
    }

    fn finish_string_consume(&mut self) -> ErrorResult {
        if self.current_string.is_empty() {
            return Ok(());
        }
        let string = take(&mut self.current_string);
        self.vec.push((string, self.current_offset));
        Ok(())
    }
}

impl VectorWriter {
    pub fn get_strings(&mut self) -> Vec<(String, u64)> {
        take(&mut self.vec)
    }
}

pub struct JsonWriter<T> {
    writer: T,
    current_offset: u64,
    is_start_writing: bool,
    is_first_element: bool,
}

impl<T> StringWriter for JsonWriter<T>
where
    T: Write,
{
    fn start_string_consume(&mut self, string: Vec<u8>, offset: u64) -> ErrorResult {
        self.current_offset = offset;
        for ch in string.into_iter() {
            self.write_chars_to_writer(ch)?;
        }
        Ok(())
    }

    fn write_char(&mut self, c: char) -> ErrorResult {
        self.write_chars_to_writer(c as u8)
    }

    fn finish_string_consume(&mut self) -> ErrorResult {
        self.writer.write_all(b"\",")?;
        self.writer
            .write_all(format!("{}", self.current_offset).as_bytes())?;
        self.writer.write_all(b"]")?;
        self.is_start_writing = false;
        Ok(())
    }
}

impl<T> JsonWriter<T>
where
    T: Write,
{
    pub fn new(writer: T) -> Self {
        JsonWriter {
            writer,
            current_offset: 0,
            is_start_writing: false,
            is_first_element: true,
        }
    }

    pub fn finish(&mut self) -> ErrorResult {
        self.writer.write_all(b"]")?;
        Ok(())
    }

    fn write_chars_to_writer(&mut self, c: u8) -> ErrorResult {
        if !self.is_start_writing {
            self.is_start_writing = true;
            if self.is_first_element {
                // Start writing the first element, needs to write `[["`
                self.writer.write_all(b"[[\"")?;
                self.is_first_element = false;
            } else {
                // Start writing current string, needs to write `,["`
                self.writer.write_all(b",[\"")?;
            }
        }
        let v = self.escape_json_character(c);
        self.writer.write_all(&v)?;
        Ok(())
    }

    fn escape_json_character(&self, c: u8) -> Vec<u8> {
        match c as char {
            '\n' => b"\\n".to_vec(),
            '\t' => b"\\t".to_vec(),
            '\r' => b"\\r".to_vec(),
            '"' => b"\\\"".to_vec(),
            '\\' => b"\\\\".to_vec(),
            _ => vec![c],
        }
    }
}
