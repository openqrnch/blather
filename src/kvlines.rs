use std::convert::From;
use std::fmt;

#[cfg(feature = "bytes")]
use bytes::{BufMut, BytesMut};

use crate::err::Error;

#[derive(Debug, Clone, Default)]
pub struct KeyValue {
  key: String,
  value: String
}

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

  pub fn get_inner(&self) -> &Vec<KeyValue> {
    &self.lines
  }

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
  #[cfg(feature = "bytes")]
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

  /// Consume the Params buffer and return the internal parameters HashMap.
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
