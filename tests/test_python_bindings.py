import os
from pathlib import Path
from uuid import uuid4

import pytest

import rust_strings


@pytest.fixture
def temp_file(tmp_path: Path) -> Path:
    file = tmp_path / str(uuid4())
    yield file
    os.remove(file)


def test_bytes():
    extracted = rust_strings.strings(bytes=b"test\x00")
    assert extracted == [("test", 0)]


def test_bytes_with_offset():
    extracted = rust_strings.strings(bytes=b"\x00test")
    assert extracted == [("test", 1)]


def test_bytes_multiple():
    extracted = rust_strings.strings(bytes=b"\x00test\x00test")
    assert extracted == [("test", 1), ("test", 6)]


def test_file(temp_file: Path):
    temp_file.write_bytes(b"test\x00")
    extracted = rust_strings.strings(file_path=temp_file)
    assert extracted == [("test", 0)]
