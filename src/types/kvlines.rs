//! A key/value pair list with stable ordering and non-unique keys.

use std::convert::From;
use std::fmt;

use bytes::{BufMut, BytesMut};

use crate::err::Error;

/// Representation of a key/value pair in `KVLines`.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct KeyValue {
  key: String,
  value: String
}

/// Ordered list of key/value pairs, with no uniqueness constraint for the
/// keys.
#[derive(Debug, Clone, Default)]
pub struct KVLines {
  lines: Vec<KeyValue>
}

impl KVLines {
  /// Create a new empty parameters object.
  pub fn new() -> Self {
    KVLines {
      ..Default::default()
    }
  }

  /// Reset all the lines.
  pub fn clear(&mut self) {
    self.lines.clear();
  }

  /// Get a reference to the inner vector of [`KeyValue`]'s.
  pub fn get_inner(&self) -> &Vec<KeyValue> {
    &self.lines
  }

  /// Append a key/value entry to the end of the list.
  pub fn append<T: ToString, U: ToString>(&mut self, key: T, value: U) {
    self.lines.push(KeyValue {
      key: key.to_string(),
      value: value.to_string()
    });
  }

  /// Calculate the size of the buffer in serialized form.
  /// Each entry will be a newline terminated utf-8 line.
  /// Last line will be a single newline character.
  pub fn calc_buf_size(&self) -> usize {
    let mut size = 0;
    for n in &self.lines {
      size += n.key.len() + 1; // including ' '
      size += n.value.len() + 1; // including '\n'
    }
    size + 1 // terminating '\n'
  }


  /// Serialize object into a `Vec<u8>` buffer suitable for transmission.
  pub fn serialize(&self) -> Result<Vec<u8>, Error> {
    let mut buf = Vec::new();

    for n in &self.lines {
      let k = n.key.as_bytes();
      let v = n.value.as_bytes();
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
  pub fn encoder_write(&self, buf: &mut BytesMut) -> Result<(), Error> {
    // Calculate the required buffer size
    let size = self.calc_buf_size();

    // Reserve space
    buf.reserve(size);

    // Write data to output buffer
    for n in &self.lines {
      buf.put(n.key.as_bytes());
      buf.put_u8(b' ');
      buf.put(n.value.as_bytes());
      buf.put_u8(b'\n');
    }
    buf.put_u8(b'\n');

    Ok(())
  }

  /// Consume the Params buffer and return the internal key/value list as a
  /// `Vec<KeyValue>`
  pub fn into_inner(self) -> Vec<KeyValue> {
    self.lines
  }
}

impl From<Vec<KeyValue>> for KVLines {
  fn from(lines: Vec<KeyValue>) -> Self {
    KVLines { lines }
  }
}


impl From<Vec<(String, String)>> for KVLines {
  fn from(lines: Vec<(String, String)>) -> Self {
    let mut out = KVLines { lines: Vec::new() };
    for (key, value) in lines {
      out.append(key, value);
    }
    out
  }
}

impl fmt::Display for KVLines {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    let mut kvlist = Vec::new();
    for n in &self.lines {
      kvlist.push(format!("{}={}", n.key, n.value));
    }
    write!(f, "{{{}}}", kvlist.join(","))
  }
}

// vim: set ft=rust et sw=2 ts=2 sts=2 cinoptions=2 tw=79 :
