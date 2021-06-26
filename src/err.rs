//! Error types and error management functions.

use std::fmt;

use tokio::io;

/// Error that `blather` can emit.
#[derive(Debug, PartialEq)]
pub enum Error {
  /// The requiested key was not found.
  KeyNotFound(String),

  /// The input format of a buffer was incorrect.
  BadFormat(String),

  /// Unable to serialize a buffer.
  SerializeError(String),

  /// A `std::io` or `tokio::io` error has occurred.
  IO(String),

  /// Something occurred which was unexpected in the current state.
  BadState(String),

  /// The specified size is invalid, or invalid in a specific context.
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
