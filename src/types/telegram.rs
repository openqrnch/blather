//! Telegrams are objects that contain a _topic_ and a set of zero or more
//! parameters.  They can be serialized into a line-based format for
//! transmission over a network link.

use std::collections::{HashMap, HashSet};
use std::fmt;
use std::str::FromStr;

use bytes::{BufMut, BytesMut};

use crate::err::Error;

use super::params::Params;
use super::validators::validate_topic;

/// Representation of a Telegram; a buffer which contains a _topic_ and a set
/// of key/value parameters.
///
/// Internally the key/value parameters are represented by a [`Params`]
/// structure.
#[derive(Debug, Clone, Default)]
pub struct Telegram {
  topic: Option<String>,
  params: Params
}

impl Telegram {
  /// Create a new telegram object, with an unset topic.
  ///
  /// Note that a telegram object without a topic is invalid.
  /// [`set_topic()`](Self::set_topic) must be called to set a topic to make
  /// the object valid.  Use [`new_topic()`](Self::new_topic) to create a new
  /// Telegram object with a topic.
  pub fn new() -> Self {
    Telegram {
      ..Default::default()
    }
  }


  /// Create a new telegram object with a topic.
  ///
  /// ```
  /// use blather::Telegram;
  /// fn main() {
  ///   let mut tg = Telegram::new_topic("Hello").unwrap();
  ///   assert_eq!(tg.get_topic(), Some("Hello"));
  /// }
  /// ```
  pub fn new_topic(topic: &str) -> Result<Self, Error> {
    validate_topic(topic)?;
    Ok(Telegram {
      topic: Some(topic.to_string()),
      ..Default::default()
    })
  }


  /// Clear topic and internal parameters buffer.
  ///
  /// ```
  /// use blather::Telegram;
  /// fn main() {
  ///   let mut tg = Telegram::new();
  ///   tg.add_param("cat", "meow");
  ///   assert_eq!(tg.num_params(), 1);
  ///   tg.clear();
  ///   assert_eq!(tg.num_params(), 0);
  /// }
  /// ```
  pub fn clear(&mut self) {
    self.topic = None;
    self.params.clear();
  }


  /// Return the number of key/value parameters in the Telegram object.
  ///
  /// # Examples
  /// ```
  /// use blather::Telegram;
  /// fn main() {
  ///   let mut tg = Telegram::new();
  ///   assert_eq!(tg.num_params(), 0);
  ///   tg.add_param("cat", "meow");
  ///   assert_eq!(tg.num_params(), 1);
  /// }
  /// ```
  ///
  /// # Notes
  /// This is a wrapper around [`Params::len()`](crate::Params::len).
  pub fn num_params(&self) -> usize {
    self.params.len()
  }


  /// Get a reference to the internal parameters object.
  pub fn get_params(&self) -> &Params {
    &self.params
  }


  /// Get a mutable reference to the inner [`Params`](crate::Params) object.
  ///
  /// ```
  /// use blather::Telegram;
  /// fn main() {
  ///   let mut tg = Telegram::new();
  ///   tg.add_param("cat", "meow");
  ///   assert_eq!(tg.num_params(), 1);
  ///   tg.get_params_mut().clear();
  ///   assert_eq!(tg.num_params(), 0);
  /// }
  /// ```
  pub fn get_params_mut(&mut self) -> &mut Params {
    &mut self.params
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
  ///
  /// # Examples
  /// ```
  /// use blather::{Telegram, Error};
  /// fn main() {
  ///   let mut tg = Telegram::new();
  ///   assert_eq!(tg.set_topic("Hello"), Ok(()));
  ///
  ///   let e = Error::BadFormat("Invalid topic character".to_string());
  ///   assert_eq!(tg.set_topic("Hell o"), Err(e));
  /// }
  /// ```
  pub fn set_topic(&mut self, topic: &str) -> Result<(), Error> {
    validate_topic(topic)?;
    self.topic = Some(topic.to_string());
    Ok(())
  }


  /// Get a reference to the topic string, or None if topic is not been set.
  ///
  /// # Examples
  /// ```
  /// use blather::{Telegram, Error};
  /// fn main() {
  ///   let tg = Telegram::new_topic("shoe0nhead").unwrap();
  ///   assert_eq!(tg.get_topic(), Some("shoe0nhead"));
  ///
  ///   let tg = Telegram::new();
  ///   assert_eq!(tg.get_topic(), None);
  /// }
  /// ```
  pub fn get_topic(&self) -> Option<&str> {
    if let Some(t) = &self.topic {
      Some(t)
    } else {
      None
    }
  }


  /// Add a parameter to the telegram.
  ///
  /// The `key` and `value` parameters are generic over the trait `ToString`,
  /// allowing a polymorphic behavior.
  ///
  /// # Examples
  /// ```
  /// use blather::Telegram;
  /// fn main() {
  ///   let mut tg = Telegram::new();
  ///   tg.add_param("integer", 42).unwrap();
  ///   tg.add_param("string", "hello").unwrap();
  /// }
  /// ```
  ///
  /// # Notes
  /// - This is a thin wrapper around
  ///   [`Params::add_param()`](crate::Params::add_param).
  pub fn add_param<T: ToString, U: ToString>(
    &mut self,
    key: T,
    value: U
  ) -> Result<(), Error> {
    self.params.add_param(key, value)
  }


  /// Add a string parameter to the telegram.
  ///
  /// # Notes
  /// - This function exists primarily for parity with a C++ library; it is
  ///   just a wrapper around [`add_param()`](Self::add_param), which is
  ///   recommended over `add_str()`.
  pub fn add_str(&mut self, key: &str, value: &str) -> Result<(), Error> {
    self.add_param(key, value)
  }


  /// Add parameter where the value is generated from an iterator over a
  /// string container, where entries will be comma-separated.
  ///
  /// ```
  /// use blather::Telegram;
  /// fn main() {
  ///   let mut tg = Telegram::new();
  ///   tg.add_strit("Cat", &["meow", "paws", "tail"]).unwrap();
  ///   assert_eq!(tg.get_str("Cat"), Some("meow,paws,tail"));
  /// }
  /// ```
  ///
  /// # Notes
  /// - This is a thin wrapper for
  ///   [`Params::add_strit()`](crate::Params::add_strit).
  pub fn add_strit<I, S>(&mut self, key: &str, c: I) -> Result<(), Error>
  where
    I: IntoIterator<Item = S>,
    S: AsRef<str>
  {
    self.params.add_strit(key, c)
  }


  /// Add a boolean value to Telegram object.
  ///
  /// # Notes
  /// - This is a thin wrapper around
  ///   [`Params::add_bool()`](crate::Params::add_bool).
  pub fn add_bool<K: ToString>(
    &mut self,
    key: K,
    value: bool
  ) -> Result<(), Error> {
    self.params.add_bool(key, value)
  }


  /// Check whether a parameter exists in Telegram object.
  ///
  /// Returns `true` is the key exists, and `false` otherwise.
  pub fn have_param(&self, key: &str) -> bool {
    self.params.have(key)
  }


  /// Get a parameter.  Fail if the parameter does not exist.
  ///
  /// # Notes
  /// - This is a thin wrapper around
  ///   [`Params::get_param()`](crate::Params::get_param).
  pub fn get_param<T: FromStr>(&self, key: &str) -> Result<T, Error> {
    self.params.get_param(key)
  }


  /// Get a parameter.  Return a default value if the parameter does not
  /// exist.
  ///
  /// # Notes
  /// - This is a thin wrapper around
  ///   [`Params::get_param_def()`](crate::Params::get_param_def).
  pub fn get_param_def<T: FromStr>(
    &self,
    key: &str,
    def: T
  ) -> Result<T, Error> {
    self.params.get_param_def(key, def)
  }


  /// Get a string representation of a parameter.  Return `None` is parameter
  /// does not exist.
  ///
  /// # Notes
  /// - This is a thin wrapper around
  ///   [`Params::get_str()`](crate::Params::get_str)
  pub fn get_str(&self, key: &str) -> Option<&str> {
    self.params.get_str(key)
  }


  /// Get a string representation of a parameter.  Returns a default value is
  /// the parameter does not exist.
  ///
  /// # Notes
  /// - This is a thin wrapper around
  ///   [`Params::get_str_def()`](crate::Params::get_str_def)
  pub fn get_str_def<'a>(&'a self, key: &str, def: &'a str) -> &'a str {
    self.params.get_str_def(key, def)
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
  /// # Notes
  /// - This function uses the `FromStr` trait on the return-type so it
  ///   technically isn't limited to integers.
  /// - The method exists to mimic a C++ library.  It is recommeded that
  ///   applications use [`Telegram::get_param()`](Self::get_param) instead.
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


  /// Return a boolean value.  Return error if parameter does not exist.
  ///
  /// If a value exist but can not be parsed as a boolean value the error
  /// `Error::BadFormat` will be returned.
  ///
  /// # Notes
  /// - This is a thing wrapper around
  ///   [`Params::get_bool()`](crate::Params::get_bool).
  pub fn get_bool(&self, key: &str) -> Result<bool, Error> {
    self.params.get_bool(key)
  }


  /// Return a boolean value.  Return a default value if parameter does not
  /// exist.
  ///
  /// # Notes
  /// - This is a thing wrapper around
  ///   [`Params::get_bool()`](crate::Params::get_bool).
  pub fn get_bool_def(&self, key: &str, def: bool) -> Result<bool, Error> {
    self.params.get_bool_def(key, def)
  }


  /// Parse the value of a key as a comma-separated list of strings and return
  /// it as a `Vec<String>`.  Only non-empty entries are returned.
  ///
  /// # Notes
  /// - This is a thin wrapper around
  ///   [`Params::get_strvec()`](crate::Params::get_strvec).
  pub fn get_strvec(&self, key: &str) -> Result<Vec<String>, Error> {
    self.params.get_strvec(key)
  }

  /// Parse the value of a key as a comma-separated list of strings and return
  /// it as a `HashSet<String>`.  Only non-empty entries are returned.
  ///
  /// # Notes
  /// - This is a thin wrapper around
  ///   [`Params::get_hashset()`](crate::Params::get_hashset).
  pub fn get_hashset(&self, key: &str) -> Result<HashSet<String>, Error> {
    self.params.get_hashset(key)
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


  /// Serialize `Telegram` into a vector of bytes for transmission.
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
