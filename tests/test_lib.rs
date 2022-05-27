use rust_strings::{dump_strings, strings, BytesConfig, Encoding, FileConfig};
use std::io::{Read, Write};
use tempfile::NamedTempFile;

#[test]
fn test_bytes_config() {
    let config = BytesConfig::new(vec![116, 101, 115, 116, 0, 0]);
    let extracted = strings(&config).unwrap();
    assert_eq!(vec![(String::from("test"), 0)], extracted);
}

#[test]
fn test_bytes_config_bytes_array() {
    let config = BytesConfig::new(b"test\x00".to_vec());
    let extracted = strings(&config).unwrap();
    assert_eq!(vec![(String::from("test"), 0)], extracted);
}

#[test]
fn test_bytes_config_offset() {
    let config = BytesConfig::new(vec![0, 116, 101, 115, 116]);
    let extracted = strings(&config).unwrap();
    assert_eq!(vec![(String::from("test"), 1)], extracted);
}

#[test]
fn test_bytes_config_min_length() {
    let config = BytesConfig::new(vec![116, 101, 115, 116, 0, 0, 116, 101, 115]).with_min_length(4);
    let extracted = strings(&config).unwrap();
    assert_eq!(vec![(String::from("test"), 0)], extracted);
}

#[test]
fn test_bytes_config_multiple_strings() {
    let config = BytesConfig::new(vec![116, 101, 115, 116, 0, 0, 116, 101, 115]).with_min_length(3);
    let extracted = strings(&config).unwrap();
    assert_eq!(
        vec![(String::from("test"), 0), (String::from("tes"), 6)],
        extracted
    );
}

#[test]
fn test_file_config() {
    let mut file = NamedTempFile::new().unwrap();
    file.write_all(b"test\x00").unwrap();

    let path = file.path();
    let config = FileConfig::new(path);
    let extracted = strings(&config).unwrap();
    assert_eq!(vec![(String::from("test"), 0)], extracted);
}

#[test]
fn test_utf16le() {
    let config =
        BytesConfig::new(b"t\x00e\x00s\x00t\x00\x00\x00".to_vec()).with_encoding(Encoding::UTF16LE);
    let extracted = strings(&config).unwrap();
    assert_eq!(vec![(String::from("test"), 0)], extracted);
}

#[test]
fn test_utf16be() {
    let config = BytesConfig::new(b"\x00t\x00e\x00s\x00t\x00\x00\x00".to_vec())
        .with_encoding(Encoding::UTF16BE);
    let extracted = strings(&config).unwrap();
    assert_eq!(vec![(String::from("test"), 0)], extracted);
}

#[test]
fn test_multiple_encodings() {
    let config = BytesConfig::new(b"ascii\x01t\x00e\x00s\x00t\x00\x00\x00".to_vec())
        .with_encoding(Encoding::ASCII)
        .with_encoding(Encoding::UTF16LE);
    let extracted = strings(&config).unwrap();
    assert_eq!(
        vec![(String::from("ascii"), 0), (String::from("test"), 6)],
        extracted
    );
}

#[test]
fn test_json_dump() {
    let file = NamedTempFile::new().unwrap();
    let config = BytesConfig::new(b"\x00\x00test\"\n\tmore\x00\x00".to_vec());

    let path = file.path().to_path_buf();
    dump_strings(&config, path).unwrap();
    let mut string = String::new();
    file.as_file().read_to_string(&mut string).unwrap();
    assert_eq!(string, String::from("[[\"test\\\"\\n\\tmore\",2]]"));
}

#[test]
fn test_json_dump_multiple_strings() {
    let file = NamedTempFile::new().unwrap();
    let config = BytesConfig::new(b"\x00\x00test\"\n\tmore\x00\x00more text over here".to_vec());

    let path = file.path().to_path_buf();
    dump_strings(&config, path).unwrap();
    let mut string = String::new();
    file.as_file().read_to_string(&mut string).unwrap();
    assert_eq!(
        string,
        String::from("[[\"test\\\"\\n\\tmore\",2],[\"more text over here\",15]]")
    );
}
