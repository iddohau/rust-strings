# Rust Strings

rust-strings is a rust library for extracting strings from binary data. \
It also have python bindings.

## Installation

Use the package manager [pip](https://pip.pypa.io/en/stable/) to install foobar.

```bash
pip install rust-strings
```

## Usage

```python
import rust_strings

# returns list of strings with their offsets
rust_strings.strings(bytes=b"test\x00\x00", min_length=4, encoding="ascii")
# [("test", 0)]

# you can also extract from file
rust_strings.strings(file_path="/bin/ls", min_length=4, encoding="ascii")
```

## Contributing
Pull requests are welcome. For major changes, please open an issue first to discuss what you would like to change.

Please make sure to update tests as appropriate.

## License
[MIT](https://choosealicense.com/licenses/mit/)