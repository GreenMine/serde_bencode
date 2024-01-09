use std::fmt::Display;

use serde::{de, ser};

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    Message(String),

    ExpectedNumber,
    ExpectedString,
    ExpectedList,
    ExpectedDictionary,
    ExpectedEnd,
    InvalidString,
    TypeNotSupported,
    Syntax,
    Eof,
}

impl ser::Error for Error {
    fn custom<T>(msg: T) -> Self
    where
        T: Display,
    {
        Error::Message(msg.to_string())
    }
}

impl de::Error for Error {
    fn custom<T>(msg: T) -> Self
    where
        T: Display,
    {
        Error::Message(msg.to_string())
    }
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::Message(m) => f.write_str(m),
            Error::Eof => write!(f, "unexpected end of file"),
            Error::ExpectedNumber => write!(f, "expected number"),
            Error::ExpectedString => write!(f, "expected string"),
            Error::ExpectedList => write!(f, "expected list"),
            Error::ExpectedDictionary => write!(f, "expected dictionary"),
            Error::TypeNotSupported => write!(f, "type is not supported in BENCODE format"),
            Error::ExpectedEnd => write!(f, "expected end"),
            Error::InvalidString => write!(f, "string is not valid"),
            Error::Syntax => write!(f, "syntax error"),
        }
    }
}

impl std::error::Error for Error {}
