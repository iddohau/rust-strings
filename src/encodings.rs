use std::fmt;
use std::error::Error;

#[derive(Debug)]
pub enum Encoding {
    ASCII,
    UTF16LE,
}

impl fmt::Display for Encoding {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(Debug)]
pub struct EncodingNotFoundError {
    encoding: String
}

impl fmt::Display for EncodingNotFoundError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Encoding not found: {:?}", self.encoding)
    }
}

impl EncodingNotFoundError {
    fn new(encoding: String) -> Self {
        EncodingNotFoundError { encoding }
    }
}

impl Error for EncodingNotFoundError {}

impl Encoding {
    pub fn from_str(encoding: &str) -> Result<Encoding, EncodingNotFoundError> {
        let encoding: &str = &encoding.to_lowercase();
        match encoding {
            "utf16le" => Ok(Encoding::UTF16LE),
            "ascii" => Ok(Encoding::ASCII),
            _ => Err(EncodingNotFoundError::new(encoding.to_owned()))
        }
    }
} 
