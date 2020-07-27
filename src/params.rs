use std::fmt;
use std::str::FromStr;
use std::collections::HashMap;
use std::convert::From;

#[cfg(feature = "bytes")]
use bytes::{BytesMut, BufMut};

use crate::err::Error;

/// Key/value parameters storage.
#[derive(Debug,Clone,Default)]
pub struct Params {
  hm: HashMap<String, String>
}

impl Params {
  /// Create a new empty parameters object.
  pub fn new() -> Self {
    Params { ..Default::default() }
  }

  /// Reset all the key/values.
  pub fn clear(&mut self) {
    self.hm.clear();
  }

  pub fn get_inner(&self) -> &HashMap<String, String> {
    &self.hm
  }

  /// Add a parameter to the parameter.
  pub fn add_param<T: ToString, U: ToString>(
      &mut self,
      key: T,
      value: U
  ) {
    self.hm.insert(key.to_string(), value.to_string());
  }

  /// Add a string parameter to the parameter.
  pub fn add_str<T: ToString, U: ToString>(
      &mut self,
      key: T,
      value: U
  ) {
    self.hm.insert(key.to_string(), value.to_string());
  }

  /// Get a parameter and convert it to a requested type.
  pub fn get_param<T: FromStr>(&self, key: &str) -> Result<T, Error> {
    if let Some(val) = self.get_str(key) {
      if let Ok(v) = T::from_str(val) {
        return Ok(v);
      }
      return Err(Error::BadFormat(format!("Unable to parse value from \
parameter '{}'", key)));
    }
    Err(Error::KeyNotFound(key.to_string()))
  }

  /// Get string representation of a value for a requested key.
  /// Returns `None` if the key is not found in the inner storage.  Returns
  /// `Some(&str)` otherwise.
  pub fn get_str(&self, key: &str) -> Option<&str> {
    let kv = self.hm.get_key_value(key);
    if let Some((_k, v)) = kv {
      return Some(v);
    }
    None
  }

  /// Get a parameter and convert it to an integer type. The logic of this
  /// method is identical to `get_param()`.
  ///
  /// ```
  /// use blather::Params;
  /// fn main() {
  ///   let mut params = Params::new();
  ///   params.add_param("Num", 7);
  ///   assert_eq!(params.get_int::<usize>("Num").unwrap(), 7);
  /// }
  /// ```
  ///
  /// This method should really have some integer trait bound, but it doesn't
  /// seem to exist in the standard library.
  ///
  /// This method exists primarily to achive some sort of parity with a
  /// corresponding C++ library.
  pub fn get_int<T: FromStr>(&self, key: &str) -> Result<T, Error> {
    if let Some(val) = self.get_str(key) {
      if let Ok(v) = T::from_str(val) {
        return Ok(v);
      }
      return Err(Error::BadFormat(format!("Unable to parse numeric value from \
parameter '{}'", key)));
    }
    Err(Error::KeyNotFound(key.to_string()))
  }

  /// Try to get the value of a key and interpret it as an integer.  If the key
  /// does not exist then return a default value supplied by the caller.
  ///
  /// ```
  /// use blather::Params;
  /// fn main() {
  ///   let mut params = Params::new();
  ///   params.add_param("num", 11);
  ///   assert_eq!(params.get_int_def::<u32>("num", 5).unwrap(), 11);
  ///   assert_eq!(params.get_int_def::<u32>("nonexistent", 17).unwrap(), 17);
  /// }
  /// ```
  pub fn get_int_def<T: FromStr>(
      &self,
      key: &str,
      def: T
  ) -> Result<T, Error> {
    if let Some(val) = self.get_str(key) {
      if let Ok(v) = T::from_str(val) {
        return Ok(v);
      }
      return Err(Error::BadFormat(format!("Unable to parse numeric value from \
parameter '{}'", key)));
    }
    Ok(def)
  }

  /// Calculate the size of the buffer in serialized form.
  /// Each entry will be a newline terminated utf-8 line.
  /// Last line will be a single newline character.
  pub fn calc_buf_size(&self) -> usize {
    let mut size = 0;
    for (key, value) in &self.hm {
      size += key.len() + 1;    // including ' '
      size += value.len() + 1;  // including '\n'
    }
    size + 1  // terminating '\n'
  }

  pub fn serialize(&self) -> Result<Vec<u8>, Error> {
    let mut buf = Vec::new();

    for (key, value) in &self.hm {
      let k = key.as_bytes();
      let v = value.as_bytes();
      for a in k {
        buf.push(*a);
      }
      buf.push(b' ');
      for a in v {
        buf.push(*a);
      }
      buf.push(b'\n');
    }

    buf.push(b'\n');

    Ok(buf)
  }

  /// Write the Params to a buffer.
  #[cfg(feature = "bytes")]
  pub fn encoder_write(
      &self,
      buf: &mut BytesMut
  ) -> Result<(), Error> {
    // Calculate the required buffer size
    let size = self.calc_buf_size();

    // Reserve space
    buf.reserve(size);

    // Write data to output buffer
    for (key, value) in &self.hm {
      buf.put(key.as_bytes());
      buf.put_u8(b' ');
      buf.put(value.as_bytes());
      buf.put_u8(b'\n');
    }
    buf.put_u8(b'\n');

    Ok(())
  }

  /// Consume the Params buffer and return the internal parameters HashMap.
  pub fn into_inner(self) -> HashMap<String, String> {
    self.hm
  }
}

impl From<HashMap<String, String>> for Params {
  fn from(hm: HashMap<String, String>) -> Self {
    Params { hm }
  }
}

impl fmt::Display for Params {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    let mut kvlist = Vec::new();
    for (key, value) in &self.hm {
      kvlist.push(format!("{}={}", key, value));
    }
    write!(f, "{{{}}}", kvlist.join(","))
  }
}

// vim: set ft=rust et sw=2 ts=2 sts=2 cinoptions=2 tw=79 :
