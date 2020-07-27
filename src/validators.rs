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
    Some(c) => if !is_topic_leading_char(c) {
      return Err(Error::BadFormat("Invalid leading character".to_string()));
    }
    None => {
      return Err(Error::BadFormat("Empty or broken string".to_string()))
    }
  }
  if chars.any(|c| !is_topic_char(c)) {
    return Err(Error::BadFormat("Invalid heading".to_string()));
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
  fn broken_topic_1() {
    if let Err(e) = validate_topic("") {
      match e {
        Error::BadFormat(s) => {
          assert_eq!(s, "Empty or broken string");
        }
        _ => {
          panic!("Unexpected error");
        }
      }
    } else {
      panic!("Unexpected success");
    }
  }
}

// vim: set ft=rust et sw=2 ts=2 sts=2 cinoptions=2 tw=79 :
