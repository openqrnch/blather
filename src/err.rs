use std::fmt;

use tokio::io;

#[derive(Debug, PartialEq)]
pub enum Error {
  KeyNotFound(String),
  BadFormat(String),
  SerializeError(String),
  IO(String),
  BadState(String),
  InvalidSize(String)
}

impl std::error::Error for Error {}

impl fmt::Display for Error {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    match &*self {
      Error::KeyNotFound(s) => write!(f, "Parameter '{}' not found", s),
      Error::BadFormat(s) => write!(f, "Bad format; {}", s),
      Error::SerializeError(s) => write!(f, "Unable to serialize; {}", s),
      Error::IO(s) => write!(f, "I/O error; {}", s),
      Error::BadState(s) => {
        write!(f, "Encountred an unexpected/bad state: {}", s)
      }
      Error::InvalidSize(s) => write!(f, "Invalid size; {}", s)
    }
  }
}

impl From<io::Error> for Error {
  fn from(err: io::Error) -> Self {
    Error::IO(err.to_string())
  }
}

// vim: set ft=rust et sw=2 ts=2 sts=2 cinoptions=2 tw=79 :
