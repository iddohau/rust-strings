from typing import Optional, List, Tuple

def strings(
    file_path: Optional[str] = None,
    bytes: Optional[bytes] = None,
    min_length: int = 3,
    encodings: List[str] = ["ascii"],
    buffer_size: int = 1024 * 1024,
) -> List[Tuple[str, int]]:
    """
    Extract strings from binary file or bytes.
    :param file_path: path to file (can't be with bytes option)
    :param bytes: bytes (can't be with file_path option)
    :param min_length: strings minimum length
    :param encodings: strings encodings (default is ["ascii"])
    :param buffer_size: the buffer size to read the file (relevant only to file_path option)
    :return: list of tuples of string and offset
    :raises: raise StringsException if there is any error during string extraction
             raise EncodingNotFoundException if the function got an unsupported encondings
    """
    ...
