use pyo3::create_exception;
use pyo3::exceptions::PyException;
use pyo3::prelude::*;
use std::error::Error;
use std::path::PathBuf;

use crate::encodings::EncodingNotFoundError;
use crate::{
    strings as r_strings, BytesConfig as RustBytesConfig, Encoding as RustEncoding,
    FileConfig as RustFileConfig,
};

create_exception!(pystrings, StringsException, PyException);
create_exception!(pystrings, EncodingNotFoundException, StringsException);

impl From<EncodingNotFoundError> for PyErr {
    fn from(err: EncodingNotFoundError) -> PyErr {
        EncodingNotFoundException::new_err(format!("{}", err))
    }
}

/// Extract strings from binary file or bytes.
/// :param file_path: path to file (can't be with bytes option)
/// :param bytes: bytes (can't be with file_path option)
/// :param min_length: strings minimum length
/// :param encoding: strings encoding (default is ascii)
/// :param buffer_size: the buffer size to read the file (relevant only to file_path option)
/// :return: list of tuples of string and offset
/// :raises: raise StringsException if there is any error during string extraction
///          raise EncodingNotFoundException if the function got an unsupported enconding
#[pyfunction(
    file_path = "None",
    bytes = "None",
    min_length = "3",
    encoding = "\"ascii\"",
    buffer_size = "1024 * 1024"
)]
#[pyo3(
    text_signature = "(file_path: str = None, bytes: bytes = None, min_length: int = 3, encoding: str = \"ascii\", buffer_size: int = 1024 * 1024) -> List[Tuple[str, int]]"
)]
fn strings(
    file_path: Option<PathBuf>,
    bytes: Option<Vec<u8>>,
    min_length: usize,
    encoding: &str,
    buffer_size: usize,
) -> PyResult<Vec<(String, u64)>> {
    if matches!(file_path, Some(_)) && matches!(bytes, Some(_)) {
        return Err(StringsException::new_err(
            "You can't specify file_path and bytes",
        ));
    }
    let encoding = RustEncoding::from_str(encoding)?;
    let result: Result<Vec<(String, u64)>, Box<dyn Error>>;
    if let Some(file_path) = file_path {
        let strings_config = RustFileConfig::new(file_path)
            .with_min_length(min_length)
            .with_encoding(encoding)
            .with_buffer_size(buffer_size);
        result = r_strings(&strings_config);
    } else if let Some(bytes) = bytes {
        let strings_config = RustBytesConfig::new(bytes)
            .with_min_length(min_length)
            .with_encoding(encoding);
        result = r_strings(&strings_config);
    } else {
        return Err(StringsException::new_err(
            "You must specify file_path or bytes",
        ));
    }
    if let Err(error_message) = result {
        return Err(StringsException::new_err(format!("{}", error_message)));
    }
    return Ok(result.unwrap());
}

#[pymodule]
#[pyo3(name = "rust_strings")]
fn rust_strings(py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(strings, m)?)?;
    m.add("StringsException", py.get_type::<StringsException>())?;
    m.add(
        "EncodingNotFoundException",
        py.get_type::<EncodingNotFoundException>(),
    )?;
    Ok(())
}
