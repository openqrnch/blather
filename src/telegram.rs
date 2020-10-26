use std::collections::HashMap;
use std::fmt;
use std::str::FromStr;

#[cfg(feature = "bytes")]
use bytes::{BufMut, BytesMut};

use crate::err::Error;

use crate::params::Params;
use crate::validators::validate_topic;

/// Representation of a Telegram buffer.
#[derive(Debug, Clone, Default)]
pub struct Telegram {
  topic: Option<String>,
  params: Params
}

impl Telegram {
  /// Create a new telegram object, with an unset topic.
  ///
  /// Note that a telegram object without a topic is invalid.  `set_topic` must
  /// be called to set a topic to make the object valid.
  pub fn new() -> Self {
    Telegram {
      ..Default::default()
    }
  }

  /// Create a new telegram object with a topic.
  pub fn new_topic(topic: &str) -> Result<Self, Error> {
    validate_topic(topic)?;
    Ok(Telegram {
      topic: Some(topic.to_string()),
      ..Default::default()
    })
  }

  /// Clear topic and internal parameters buffer.
  pub fn clear(&mut self) {
    self.topic = None;
    self.params.clear();
  }

  /// Get a reference to the internal parameters object.
  pub fn get_params(&self) -> &Params {
    &self.params
  }

  /// Get a reference the the parameter's internal HashMap.
  ///
  /// Note: The inner representation of the Params object may change in the
  /// future.
  pub fn get_params_inner(&self) -> &HashMap<String, String> {
    &self.params.get_inner()
  }

  /// Set topic for telegram.
  ///
  /// Overwrites current topic is one has already been set.
  pub fn set_topic(&mut self, topic: &str) -> Result<(), Error> {
    validate_topic(topic)?;
    self.topic = Some(topic.to_string());
    Ok(())
  }

  /// Get a reference to the topic string, or None if topic is not been set.
  pub fn get_topic(&self) -> Option<&str> {
    if let Some(t) = &self.topic {
      Some(t)
    } else {
      None
    }
  }

  /// Add a parameter to the telegram.
  pub fn add_param<T: ToString, U: ToString>(
    &mut self,
    key: T,
    value: U
  ) -> Result<(), Error> {
    self.params.add_param(key, value)
  }

  /// Add a string parameter to the telegram.
  ///
  /// This function exists primarily for parity with a C++ library.  Just a
  /// wrapper around `add_param()`.
  pub fn add_str<T: ToString, U: ToString>(
    &mut self,
    key: T,
    value: U
  ) -> Result<(), Error> {
    self.add_param(key, value)
  }


  pub fn have_param(&self, key: &str) -> bool {
    self.params.have(key)
  }

  /// Get a string representation of a parameter.
  pub fn get_param<T: FromStr>(&self, key: &str) -> Result<T, Error> {
    self.params.get_param(key)
  }

  /// Get a string representation of a parameter.
  pub fn get_str(&self, key: &str) -> Option<&str> {
    self.params.get_str(key)
  }

  /// Get an integer representation of a parameter.
  ///
  /// ```
  /// use blather::Telegram;
  /// fn main() {
  ///   let mut tg = Telegram::new();
  ///   tg.add_param("Num", 7);
  ///   assert_eq!(tg.get_int::<usize>("Num").unwrap(), 7);
  /// }
  /// ```
  ///
  /// Note: This function uses the `FromStr` trait so it technically isn't
  /// limited to integers.  The method name is chosen to mimic a C++ library.
  pub fn get_int<T: FromStr>(&self, key: &str) -> Result<T, Error> {
    self.params.get_int(key)
  }

  /// Try to get the parameter value of a key and interpret it as an integer.
  /// If the key does not exist then return a default value supplied by the
  /// caller.
  ///
  /// ```
  /// use blather::Telegram;
  /// fn main() {
  ///   let mut tg = Telegram::new();
  ///   tg.add_param("num", 11);
  ///   assert_eq!(tg.get_int_def::<u32>("num", 5).unwrap(), 11);
  ///   assert_eq!(tg.get_int_def::<u32>("nonexistent", 17).unwrap(), 17);
  /// }
  /// ```
  pub fn get_int_def<T: FromStr>(
    &self,
    key: &str,
    def: T
  ) -> Result<T, Error> {
    self.params.get_int_def(key, def)
  }

  /// Calculate the size of a serialized version of this Telegram object.
  /// If no topic has been set it is simply ignored.  In the future this might
  /// change to something more dramatic, like a panic.  Telegrams should always
  /// contain a topic when transmitted.
  ///
  /// Each line is terminated by a newline character.
  /// The last line consists of a single newline character.
  pub fn calc_buf_size(&self) -> usize {
    // Calculate the required buffer size
    let mut size = 0;
    if let Some(ref h) = self.topic {
      size += h.len() + 1; // including '\n'
    }

    // Note that the Params method reserves the final terminating newline.
    size + self.params.calc_buf_size()
  }

  pub fn serialize(&self) -> Result<Vec<u8>, Error> {
    let mut buf = Vec::new();

    if let Some(ref h) = self.topic {
      // Copy topic
      let b = h.as_bytes();
      for a in b {
        buf.push(*a);
      }
      buf.push(b'\n');
    } else {
      return Err(Error::BadFormat("Missing heading".to_string()));
    }

    for (key, value) in self.get_params_inner() {
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

  /// Write the Telegram to a BytesMut buffer.
  #[cfg(feature = "bytes")]
  pub fn encoder_write(&self, buf: &mut BytesMut) -> Result<(), Error> {
    if self.topic.is_none() {
      return Err(Error::SerializeError("Missing Telegram topic".to_string()));
    }

    // Calculate the required buffer size
    let size = self.calc_buf_size();

    // Reserve space
    buf.reserve(size);

    // Write data to output buffer
    if let Some(ref b) = self.topic {
      buf.put(b.as_bytes());
    }
    buf.put_u8(b'\n');

    for (key, value) in self.get_params_inner() {
      buf.put(key.as_bytes());
      buf.put_u8(b' ');
      buf.put(value.as_bytes());
      buf.put_u8(b'\n');
    }
    buf.put_u8(b'\n');

    Ok(())
  }

  /// Consume the Telegram buffer and return the internal parameters object.
  pub fn into_params(self) -> Params {
    self.params
  }
}

impl From<String> for Telegram {
  fn from(topic: String) -> Self {
    Telegram {
      topic: Some(topic),
      ..Default::default()
    }
  }
}

impl From<Params> for Telegram {
  fn from(params: Params) -> Self {
    Telegram {
      params,
      ..Default::default()
    }
  }
}

impl From<HashMap<String, String>> for Telegram {
  fn from(hm: HashMap<String, String>) -> Self {
    Telegram {
      params: Params::from(hm),
      ..Default::default()
    }
  }
}

impl fmt::Display for Telegram {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    let topic: &str = match &self.topic {
      Some(s) => s.as_ref(),
      None => &"<None>"
    };

    write!(f, "{}:{}", topic, self.params)
  }
}

// vim: set ft=rust et sw=2 ts=2 sts=2 cinoptions=2 tw=79 :
