use std::{error, fmt, io, result, str};
use byteorder;

pub type Result<T> = result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    BadMagic(String),
    VersionNotSupported(u16),
    CompressionTypeUnknown(String),
    UnsupportedCodec(String),
    SyncMarkerMismatch,
    EOF,
    IO(io::Error),
    BadEncoding(str::Utf8Error),
    UnexpectedDecoderError(byteorder::Error),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> result::Result<(), fmt::Error> {
        match *self {
            Error::BadMagic(ref m) => {
                write!(f,
                       "bad or missing magic string, found: '{}'. Is this a sequence file?",
                       m)
            }
            Error::IO(_) => write!(f, "i/o error: {}", self),
            Error::VersionNotSupported(ref v) => write!(f, "unexpected version: '{}'", v),
            Error::SyncMarkerMismatch => write!(f, "sync marker mismatch"),
            Error::EOF => write!(f, "end of file"),
            Error::CompressionTypeUnknown(ref codec) => {
                write!(f, "unexpected compression type: '{}'", codec)
            }
            Error::UnsupportedCodec(ref codec) => write!(f, "unsupported codec: '{}'", codec),
            Error::BadEncoding(ref e) => write!(f, "utf8 error: {}", e),
            Error::UnexpectedDecoderError(ref e) => write!(f, "decoding error: {}", e),
        }
    }
}

impl error::Error for Error {
    fn description(&self) -> &str {
        match *self {
            Error::BadMagic(_) => "bad or missing magic header",
            Error::VersionNotSupported(_) => "bad version number",
            Error::SyncMarkerMismatch => "sync marker mismatch",
            Error::EOF => "End of file",
            Error::CompressionTypeUnknown(_) => "unable to decompress, unknown codec",
            Error::UnsupportedCodec(_) => "unsupported codec",
            Error::IO(ref e) => e.description(),
            Error::BadEncoding(ref e) => e.description(),
            Error::UnexpectedDecoderError(ref e) => e.description(),
        }
    }

    fn cause(&self) -> Option<&error::Error> {
        match *self {
            Error::IO(ref e) => Some(e),
            Error::BadEncoding(ref e) => Some(e),
            Error::UnexpectedDecoderError(ref e) => Some(e),
            _ => None,
        }
    }
}

impl From<io::Error> for Error {
    fn from(err: io::Error) -> Error {
        Error::IO(err)
    }
}


impl From<byteorder::Error> for Error {
    fn from(err: byteorder::Error) -> Error {
        Error::UnexpectedDecoderError(err)
    }
}

impl From<str::Utf8Error> for Error {
    fn from(err: str::Utf8Error) -> Error {
        Error::BadEncoding(err)
    }
}
