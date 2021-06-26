use crate::err::Error;

fn is_topic_leading_char(c: char) -> bool {
  c.is_alphabetic()
}

fn is_topic_char(c: char) -> bool {
  c.is_alphanumeric() || c == '_' || c == '-'
}

/// Make sure that topic string is valid.
pub fn validate_topic(topic: &str) -> Result<(), Error> {
  let mut chars = topic.chars();
  match chars.next() {
    Some(c) => {
      if !is_topic_leading_char(c) {
        return Err(Error::BadFormat(
          "Invalid leading topic character".to_string()
        ));
      }
    }
    None => return Err(Error::BadFormat("Empty or broken topic".to_string()))
  }

  if chars.any(|c| !is_topic_char(c)) {
    return Err(Error::BadFormat("Invalid topic character".to_string()));
  }
  Ok(())
}


fn is_key_char(c: char) -> bool {
  c.is_alphanumeric() || c.is_ascii_punctuation()
}

/// Make sure that a parameter key is valid.
pub fn validate_param_key(key: &str) -> Result<(), Error> {
  let mut chars = key.chars();
  match chars.next() {
    Some(c) => {
      if !is_key_char(c) {
        return Err(Error::BadFormat("Invalid key character".to_string()));
      }
    }
    None => return Err(Error::BadFormat("Empty or broken key".to_string()))
  }

  if chars.any(|c| !is_key_char(c)) {
    return Err(Error::BadFormat("Invalid key character".to_string()));
  }
  Ok(())
}


#[cfg(test)]
mod tests {
  use super::validate_topic;
  use super::Error;

  #[test]
  fn ok_topic_1() {
    assert!(validate_topic("Foobar").is_ok());
  }

  #[test]
  fn empty_topic() {
    assert_eq!(
      validate_topic(""),
      Err(Error::BadFormat("Empty or broken topic".to_string()))
    );
  }

  #[test]
  fn broken_topic_1() {
    assert_eq!(
      validate_topic("foo bar"),
      Err(Error::BadFormat("Invalid topic character".to_string()))
    );
  }

  #[test]
  fn broken_topic_2() {
    assert_eq!(
      validate_topic(" foobar"),
      Err(Error::BadFormat(
        "Invalid leading topic character".to_string()
      ))
    );
  }
}

// vim: set ft=rust et sw=2 ts=2 sts=2 cinoptions=2 tw=79 :
