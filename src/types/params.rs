//! The `Params` buffer is a set of unorderded key/value pairs, with unique
//! keys.  It's similar to a `HashMap`, but has constraints on key names and
//! offers conventions for value layouts, such as comma-separated values for
//! lists.

use std::collections::{HashMap, HashSet};
use std::convert::From;
use std::fmt;
use std::str::FromStr;

use bytes::{BufMut, BytesMut};

use super::validators::validate_param_key;

use crate::err::Error;

/// Key/value parameters storage with helper methods to make adding and getting
/// common value types slightly more ergonomic and using a plain `HashMap`.
///
/// Uses `String`s for both keys and values internally.
#[derive(Debug, Clone, Default)]
pub struct Params {
  hm: HashMap<String, String>
}

impl Params {
  /// Create a new empty parameters object.
  pub fn new() -> Self {
    Params {
      ..Default::default()
    }
  }


  /// Reset all the key/values in `Params` object.
  pub fn clear(&mut self) {
    self.hm.clear();
  }


  /// Return the number of key/value pairs in the parameter buffer.
  pub fn len(&self) -> usize {
    self.hm.len()
  }


  /// Return reference to inner HashMap.
  pub fn get_inner(&self) -> &HashMap<String, String> {
    &self.hm
  }


  /// Add a parameter to the parameter.
  ///
  /// The `key` and `value` parameters are generic over the trait `ToString`,
  /// allowing a polymorphic behavior.
  ///
  /// # Examples
  /// ```
  /// use blather::Params;
  /// fn main() {
  ///   let mut params = Params::new();
  ///   params.add_param("integer", 42).unwrap();
  ///   params.add_param("string", "hello").unwrap();
  /// }
  /// ```
  pub fn add_param<T: ToString, U: ToString>(
    &mut self,
    key: T,
    value: U
  ) -> Result<(), Error> {
    let key = key.to_string();

    validate_param_key(&key)?;

    self.hm.insert(key, value.to_string());
    Ok(())
  }


  /// Add a string parameter to the parameter.
  ///
  /// # Notes
  /// - This method exists for parity with a C++ interface and is a less
  ///   flexible version of [`add_param()`](Self::add_param), which application
  ///   should use instead.
  pub fn add_str(&mut self, key: &str, value: &str) -> Result<(), Error> {
    self.add_param(key, value)
  }


  /// Add parameter where the value is generated from an iterator over
  /// strings, where entries are comma-separated.
  ///
  /// # Examples
  /// ```
  /// use std::collections::HashSet;
  /// use blather::Params;
  /// fn main() {
  ///   let mut params = Params::new();
  ///
  ///   params.add_strit("Cat", &["meow", "paws", "tail"]).unwrap();
  ///   assert_eq!(params.get_str("Cat"), Some("meow,paws,tail"));
  ///
  ///   let v = vec!["meow", "paws", "tail"];
  ///   params.add_strit("CatToo", v.into_iter()).unwrap();
  ///   assert_eq!(params.get_str("CatToo"), Some("meow,paws,tail"));
  ///
  ///   let mut hs = HashSet::new();
  ///   hs.insert("Elena");
  ///   hs.insert("Drake");
  ///   params.add_strit("Uncharted", hs.into_iter()).unwrap();
  /// }
  /// ```
  pub fn add_strit<I, S>(&mut self, key: &str, c: I) -> Result<(), Error>
  where
    I: IntoIterator<Item = S>,
    S: AsRef<str>
  {
    let mut sv = Vec::new();
    for o in c.into_iter() {
      sv.push(o.as_ref().to_string());
    }
    self.add_param(key, sv.join(","))?;

    Ok(())
  }


  /// Add a boolean parameter.
  ///
  /// # Examples
  /// ```
  /// use blather::Params;
  /// fn main() {
  ///   let mut params = Params::new();
  ///   params.add_bool("should_be_true", true).unwrap();
  ///   params.add_bool("should_be_false", false).unwrap();
  ///   assert_eq!(params.get_bool("should_be_true"), Ok(true));
  ///   assert_eq!(params.get_bool("should_be_false"), Ok(false));
  /// }
  /// ```
  ///
  /// # Notes
  /// - Applications should not make assumptions about the specific string
  ///   value added by this function.  Do not treat boolean values as strings;
  ///   use the [`get_bool()`](Self::get_bool) method instead.
  pub fn add_bool<K: ToString>(
    &mut self,
    key: K,
    value: bool
  ) -> Result<(), Error> {
    let v = match value {
      true => "True",
      false => "False"
    };
    self.add_param(key, v)
  }


  /// Returns `true` if the parameter with `key` exists.  Returns `false`
  /// otherwise.
  pub fn have(&self, key: &str) -> bool {
    self.hm.contains_key(key)
  }


  /// Get a parameter and convert it to a requested type, fail if key isn't
  /// found.
  ///
  /// # Examples
  /// ```
  /// use blather::{Params, Error};
  /// fn main() {
  ///   let mut params = Params::new();
  ///   params.add_param("arthur", 42);
  ///   let fourtytwo = params.get_param::<u32>("arthur").unwrap();
  ///   assert_eq!(fourtytwo, 42);
  ///   let nonexist = params.get_param::<u32>("ford");
  ///   assert_eq!(nonexist, Err(Error::KeyNotFound("ford".to_string())));
  /// }
  /// ```
  pub fn get_param<T: FromStr>(&self, key: &str) -> Result<T, Error> {
    if let Some(val) = self.get_str(key) {
      if let Ok(v) = T::from_str(val) {
        return Ok(v);
      }
      return Err(Error::BadFormat(format!(
        "Unable to parse value from parameter '{}'",
        key
      )));
    }
    Err(Error::KeyNotFound(key.to_string()))
  }


  /// Get a parameter and convert it to a requested type, return a default
  /// value if key isn't found.
  ///
  /// # Examples
  /// ```
  /// use blather::Params;
  /// fn main() {
  ///   let mut params = Params::new();
  ///   let val = params.get_param_def::<u32>("nonexist", 11);
  ///   assert_eq!(val, Ok(11));
  /// }
  /// ```
  pub fn get_param_def<T: FromStr>(
    &self,
    key: &str,
    def: T
  ) -> Result<T, Error> {
    if let Some(val) = self.get_str(key) {
      if let Ok(v) = T::from_str(val) {
        return Ok(v);
      }
      return Err(Error::BadFormat(format!(
        "Unable to parse value from parameter '{}'",
        key
      )));
    }
    Ok(def)
  }


  /// Get string representation of a value for a requested key.
  /// Returns `None` if the key is not found in the inner storage.  Returns
  /// `Some(&str)` if parameter exists.
  pub fn get_str(&self, key: &str) -> Option<&str> {
    let kv = self.hm.get_key_value(key);
    if let Some((_k, v)) = kv {
      return Some(v);
    }
    None
  }


  /// Get string representation of a value for a requested key.  Returns a
  /// default value if key does not exist in parameter buffer.
  ///
  /// # Examples
  /// ```
  /// use blather::Params;
  /// fn main() {
  ///   let params = Params::new();
  ///   let e = params.get_str_def("nonexist", "elena");
  ///   assert_eq!(e, "elena");
  /// }
  /// ```
  // Lifetimes of self and def don't really go hand-in-hand, but we bound them
  // together for the sake of the return value's lifetime.
  pub fn get_str_def<'a>(&'a self, key: &str, def: &'a str) -> &'a str {
    let kv = self.hm.get_key_value(key);
    if let Some((_k, v)) = kv {
      v
    } else {
      def
    }
  }


  /// Get a parameter and convert it to an integer type.
  ///
  /// # Examples
  /// ```
  /// use blather::Params;
  /// fn main() {
  ///   let mut params = Params::new();
  ///   params.add_param("Num", 7);
  ///   assert_eq!(params.get_int::<usize>("Num").unwrap(), 7);
  /// }
  /// ```
  ///
  /// # Notes
  /// - This method exists primarily to achive some sort of parity with a
  ///   corresponding C++ library.  It is recommended that applications use
  ///   [`Params::get_param()`](Self::get_param) instead.
  // This method should really have some integer trait bound, but it doesn't
  // seem to exist in the standard library.
  pub fn get_int<T: FromStr>(&self, key: &str) -> Result<T, Error> {
    if let Some(val) = self.get_str(key) {
      if let Ok(v) = T::from_str(val) {
        return Ok(v);
      }
      return Err(Error::BadFormat(format!(
        "Unable to parse numeric value from parameter '{}'",
        key
      )));
    }
    Err(Error::KeyNotFound(key.to_string()))
  }


  /// Try to get the value of a key and interpret it as an integer.  If the key
  /// does not exist then return a default value supplied by the caller.
  ///
  /// # Examples
  /// ```
  /// use blather::Params;
  /// fn main() {
  ///   let mut params = Params::new();
  ///   params.add_param("num", 11);
  ///   assert_eq!(params.get_int_def::<u32>("num", 5).unwrap(), 11);
  ///   assert_eq!(params.get_int_def::<u32>("nonexistent", 17).unwrap(), 17);
  /// }
  /// ```
  ///
  /// # Notes
  /// - It is recommended that application use
  ///   [`Params::get_param_def()`](Self::get_param_def) instead.
  pub fn get_int_def<T: FromStr>(
    &self,
    key: &str,
    def: T
  ) -> Result<T, Error> {
    if let Some(val) = self.get_str(key) {
      if let Ok(v) = T::from_str(val) {
        return Ok(v);
      }
      return Err(Error::BadFormat(format!(
        "Unable to parse numeric value from parameter '{}'",
        key
      )));
    }
    Ok(def)
  }


  /// Get a boolean value; return error if key wasn't found.
  pub fn get_bool(&self, key: &str) -> Result<bool, Error> {
    if let Some(v) = self.get_str(key) {
      let v = v.to_ascii_lowercase();
      match v.as_ref() {
        "y" | "yes" | "t" | "true" | "1" => {
          return Ok(true);
        }
        "n" | "no" | "f" | "false" | "0" => {
          return Ok(false);
        }
        _ => {
          return Err(Error::BadFormat(
            "Unrecognized boolean value".to_string()
          ));
        }
      }
    }

    Err(Error::KeyNotFound(key.to_string()))
  }

  /// Get a boolean value; return a default value if key wasn't found.
  pub fn get_bool_def(&self, key: &str, def: bool) -> Result<bool, Error> {
    match self.get_bool(key) {
      Ok(v) => Ok(v),
      Err(Error::KeyNotFound(_)) => Ok(def),
      Err(e) => Err(e)
    }
  }


  /// Parse the value of a key as a comma-separated list of strings and return
  /// it.  Only non-empty entries are returned.
  ///
  /// # Examples
  /// ```
  /// use blather::Params;
  /// fn main() {
  ///   let mut params = Params::new();
  ///   params.add_param("csv", "elena,chloe,drake");
  ///   let sv = params.get_strvec("csv").unwrap();
  ///   assert_eq!(sv, vec!["elena", "chloe", "drake"]);
  /// }
  /// ```
  pub fn get_strvec(&self, key: &str) -> Result<Vec<String>, Error> {
    let mut ret = Vec::new();

    if let Some(v) = self.get_str(key) {
      let split = v.split(',');
      for s in split {
        if s.len() != 0 {
          ret.push(s.to_string());
        }
      }
    }

    Ok(ret)
  }


  /// Parse the value of a key as a comma-separated list of uniqie strings and
  /// return them in a HashSet.  Only non-empty entries are returned.
  ///
  /// # Examples
  /// ```
  /// use blather::Params;
  /// fn main() {
  ///   let mut params = Params::new();
  ///   params.add_param("set", "elena,chloe");
  ///   let set = params.get_hashset("set").unwrap();
  ///   assert_eq!(set.len(), 2);
  ///   assert_eq!(set.contains("elena"), true);
  ///   assert_eq!(set.contains("chloe"), true);
  ///   assert_eq!(set.contains("drake"), false);
  /// }
  /// ```
  pub fn get_hashset(&self, key: &str) -> Result<HashSet<String>, Error> {
    let mut ret = HashSet::new();

    if let Some(v) = self.get_str(key) {
      let split = v.split(',');
      for s in split {
        if s.len() != 0 {
          ret.insert(s.to_string());
        }
      }
    }

    Ok(ret)
  }


  /// Calculate the size of the buffer in serialized form.
  /// Each entry will be a newline terminated utf-8 line.
  /// Last line will be a single newline character.
  pub fn calc_buf_size(&self) -> usize {
    let mut size = 0;
    for (key, value) in &self.hm {
      size += key.len() + 1; // including ' '
      size += value.len() + 1; // including '\n'
    }
    size + 1 // terminating '\n'
  }


  /// Serialize `Params` buffer into a vector of bytes for transmission.
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
  pub fn encoder_write(&self, buf: &mut BytesMut) -> Result<(), Error> {
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

  /// Consume the Params buffer and return its internal HashMap.
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
