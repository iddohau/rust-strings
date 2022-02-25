# rust-strings

[![CI](https://github.com/iddohau/rust-strings/workflows/Rust%20Lint%20%26%20Test/badge.svg?branch=main)](https://github.com/iddohau/rust-strings/actions?query=branch=main)
![License](https://img.shields.io/github/license/iddohau/rust-strings)
![Crates.io](https://img.shields.io/crates/v/rust-strings)
[![PyPI](https://img.shields.io/pypi/v/rust-strings.svg)](https://pypi.org/project/rust-strings)

`rust-strings` is a Rust library for extracting strings from binary data. \
It also have Python bindings.

## Installation

### Python

Use the package manager [pip](https://pip.pypa.io/en/stable/) to install `rust-strings`.

```bash
pip install rust-strings
```

### Rust

`rust-strings` is available on [crates.io](https://crates.io/crates/rust-strings) and can be included in your Cargo enabled project like this:

```bash
[dependencies]
rust-strings = "0.1.0"
```

## Usage

### Python

```python
import rust_strings

# Get all ascii strings from file with minimun length of string
rust_strings.strings(file_path="/bin/ls", min_length=3)
# [('ELF', 1),
#  ('/lib64/ld-linux-x86-64.so.2', 680),
#  ('GNU', 720),
#  ('.<O', 725),
#  ('GNU', 756),
# ...]

# You can also set buffer size when reading from file (default is 1mb)
rust_strings.strings(file_path="/bin/ls", min_length=5, buffer_size=1024)

# You can set encoding if you need (default is 'ascii', options are 'utf-16le', 'utf-16be')
rust_strings.strings(file_path=r"C:\Windows\notepad.exe", min_length=5, encodings=["utf-16le"])

# You can set multiple encoding
rust_strings.strings(file_path=r"C:\Windows\notepad.exe", min_length=5, encodings=["ascii", "utf-16le"])

# You can also pass bytes instead of file_path
rust_strings.strings(bytes=b"test\x00\x00", min_length=4, encodings=["ascii"])
# [("test", 0)]

# You can also dump to json file
rust_strings.dump_strings("strings.json", bytes=b"test\x00\x00", min_length=4, encodings=["ascii"])
# `strings.json` content:
# [["test", 0]]
```

### Rust

Full documentation available in [docs.rs](https://docs.rs/rust-strings)

```rust
use rust_strings::{FileConfig, BytesConfig, strings, dump_strings, Encoding};
let config = FileConfig::new("/bin/ls").with_min_length(5);
let extracted_strings = strings(&config);

// Extract utf16le strings
let config = FileConfig::new("C:\\Windows\\notepad.exe")
    .with_min_length(15)
    .with_encoding(Encoding::UTF16LE);
let extracted_strings = strings(&config);

// Extract ascii and utf16le strings
let config = FileConfig::new("C:\\Windows\\notepad.exe")
    .with_min_length(15)
    .with_encoding(Encoding::ASCII)
    .with_encoding(Encoding::UTF16LE);
let extracted_strings = strings(&config);

let config = BytesConfig::new(b"test\x00".to_vec());
let extracted_strings = strings(&config);
assert_eq!(vec![(String::from("test"), 0)], extracted_strings.unwrap());

// Dump strings into `strings.json` file.
let config = BytesConfig::new(b"test\x00".to_vec());
dump_strings(&config, PathBuf::from("strings.json"));
```

## Contributing
Pull requests are welcome. For major changes, please open an issue first to discuss what you would like to change.

Please make sure to update tests as appropriate.

## License
[MIT](https://choosealicense.com/licenses/mit/)