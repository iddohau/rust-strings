use std::error::Error;
use std::fmt;
use std::str::FromStr;

#[derive(Debug, Copy, Clone)]
pub enum Encoding {
    ASCII,
    UTF16LE,
    UTF16BE,
}

impl fmt::Display for Encoding {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(Debug)]
pub struct EncodingNotFoundError {
    encoding: String,
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

impl FromStr for Encoding {
    type Err = EncodingNotFoundError;

    fn from_str(encoding: &str) -> Result<Self, Self::Err> {
        let encoding: &str = &encoding.to_lowercase();
        match encoding {
            "utf-16le" => Ok(Encoding::UTF16LE),
            "utf-16be" => Ok(Encoding::UTF16BE),
            "ascii" => Ok(Encoding::ASCII),
            "utf8" => Ok(Encoding::ASCII),
            "utf-8" => Ok(Encoding::ASCII),
            _ => Err(EncodingNotFoundError::new(encoding.to_owned())),
        }
    }
}
