use std::fmt;

#[derive(Debug)]
pub enum Error {
  KeyNotFound(String),
  BadFormat(String),
  SerializeError(String)
}

impl std::error::Error for Error { }

impl fmt::Display for Error {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    match &*self {
      Error::KeyNotFound(s) => {
        write!(f, "Parameter '{}' not found", s)
      }
      Error::BadFormat(s) => {
        write!(f, "Bad format; {}", s)
      }
      Error::SerializeError(s) => {
        write!(f, "Unable to serialize; {}", s)
      }
    }
  }
}

// vim: set ft=rust et sw=2 ts=2 sts=2 cinoptions=2 tw=79 :
