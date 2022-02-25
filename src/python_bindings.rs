use pyo3::create_exception;
use pyo3::exceptions::PyException;
use pyo3::prelude::*;
use std::error::Error;
use std::path::PathBuf;
use std::str::FromStr;

use crate::encodings::EncodingNotFoundError;
use crate::{
    dump_strings as r_dump_strings, strings as r_strings, BytesConfig as RustBytesConfig,
    Encoding as RustEncoding, ErrorResult, FileConfig as RustFileConfig,
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
/// :param encoding: strings encoding (default is ["ascii"])
/// :param buffer_size: the buffer size to read the file (relevant only to file_path option)
/// :return: list of tuples of string and offset
/// :raises: raise StringsException if there is any error during string extraction
///          raise EncodingNotFoundException if the function got an unsupported encondings
#[pyfunction(
    file_path = "None",
    bytes = "None",
    min_length = "3",
    encodings = "vec![\"ascii\"]",
    buffer_size = "1024 * 1024"
)]
#[pyo3(
    text_signature = "(file_path: Optional[str] = None, bytes: Optional[bytes] = None, min_length: int = 3, encoding: List[str] = [\"ascii\"], buffer_size: int = 1024 * 1024) -> List[Tuple[str, int]]"
)]
fn strings(
    file_path: Option<PathBuf>,
    bytes: Option<Vec<u8>>,
    min_length: usize,
    encodings: Vec<&str>,
    buffer_size: usize,
) -> PyResult<Vec<(String, u64)>> {
    if matches!(file_path, Some(_)) && matches!(bytes, Some(_)) {
        return Err(StringsException::new_err(
            "You can't specify file_path and bytes",
        ));
    }
    let encodings = encodings
        .iter()
        .map(|e| RustEncoding::from_str(e))
        .collect::<Result<Vec<RustEncoding>, _>>()?;
    let result: Result<Vec<(String, u64)>, Box<dyn Error>>;
    if let Some(file_path) = file_path {
        let strings_config = RustFileConfig::new(file_path.to_str().unwrap())
            .with_min_length(min_length)
            .with_encodings(encodings)
            .with_buffer_size(buffer_size);
        result = r_strings(&strings_config);
    } else if let Some(bytes) = bytes {
        let strings_config = RustBytesConfig::new(bytes)
            .with_min_length(min_length)
            .with_encodings(encodings);
        result = r_strings(&strings_config);
    } else {
        return Err(StringsException::new_err(
            "You must specify file_path or bytes",
        ));
    }
    if let Err(error_message) = result {
        return Err(StringsException::new_err(format!("{}", error_message)));
    }
    Ok(result.unwrap())
}

/// Dump strings from binary file or bytes to json file.
/// :param output_file: path to file to dump into
/// :param file_path: path to file (can't be with bytes option)
/// :param bytes: bytes (can't be with file_path option)
/// :param min_length: strings minimum length
/// :param encoding: strings encoding (default is ["ascii"])
/// :param buffer_size: the buffer size to read the file (relevant only to file_path option)
/// :return: list of tuples of string and offset
/// :raises: raise StringsException if there is any error during string extraction
///          raise EncodingNotFoundException if the function got an unsupported encondings
#[pyfunction(
    file_path = "None",
    bytes = "None",
    min_length = "3",
    encodings = "vec![\"ascii\"]",
    buffer_size = "1024 * 1024"
)]
#[pyo3(
    text_signature = "(output_file: str, file_path: Optional[str] = None, bytes: Optional[bytes] = None, min_length: int = 3, encoding: List[str] = [\"ascii\"], buffer_size: int = 1024 * 1024) -> None"
)]
fn dump_strings(
    output_file: PathBuf,
    file_path: Option<PathBuf>,
    bytes: Option<Vec<u8>>,
    min_length: usize,
    encodings: Vec<&str>,
    buffer_size: usize,
) -> PyResult<()> {
    if matches!(file_path, Some(_)) && matches!(bytes, Some(_)) {
        return Err(StringsException::new_err(
            "You can't specify file_path and bytes",
        ));
    }
    let encodings = encodings
        .iter()
        .map(|e| RustEncoding::from_str(e))
        .collect::<Result<Vec<RustEncoding>, _>>()?;
    let result: ErrorResult;
    if let Some(file_path) = file_path {
        let strings_config = RustFileConfig::new(file_path.to_str().unwrap())
            .with_min_length(min_length)
            .with_encodings(encodings)
            .with_buffer_size(buffer_size);
        result = r_dump_strings(&strings_config, output_file);
    } else if let Some(bytes) = bytes {
        let strings_config = RustBytesConfig::new(bytes)
            .with_min_length(min_length)
            .with_encodings(encodings);
        result = r_dump_strings(&strings_config, output_file);
    } else {
        return Err(StringsException::new_err(
            "You must specify file_path or bytes",
        ));
    }
    if let Err(error_message) = result {
        return Err(StringsException::new_err(format!("{}", error_message)));
    }
    Ok(())
}

#[pymodule]
#[pyo3(name = "rust_strings")]
fn rust_strings(py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(strings, m)?)?;
    m.add_function(wrap_pyfunction!(dump_strings, m)?)?;
    m.add("StringsException", py.get_type::<StringsException>())?;
    m.add(
        "EncodingNotFoundException",
        py.get_type::<EncodingNotFoundException>(),
    )?;
    Ok(())
}
