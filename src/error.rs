use crate::response::Response;
use std::fmt;

pub enum Error {
    Empty,
    TooLong,
    InvalidUtf8,
    UnknownCommand(String),
    BadArgs(String),
}

impl Error {
    pub fn to_response(&self) -> Response {
        match self {
            Error::Empty => Response::Error("ERR empty command".into()),
            Error::TooLong => Response::Error("ERR command too long".into()),
            Error::InvalidUtf8 => Response::Error("ERR invalid UTF-8".into()),
            Error::UnknownCommand(cmd) => Response::Error(format!("ERR unknown command '{}'", cmd)),
            Error::BadArgs(msg) => Response::Error(format!("ERR {}", msg)),
        }
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::Empty => write!(f, "ERR empty command"),
            Error::TooLong => write!(f, "ERR command too long"),
            Error::InvalidUtf8 => write!(f, "ERR invalid UTF-8"),
            Error::UnknownCommand(cmd) => write!(f, "ERR unknown command: {}", cmd),
            Error::BadArgs(msg) => write!(f, "ERR {}", msg),
        }
    }
}
