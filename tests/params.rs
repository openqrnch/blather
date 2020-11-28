use blather::{Error, Params};


#[test]
fn string() {
  let mut msg = Params::new();

  msg.add_str("Foo", "bar").unwrap();
  assert_eq!(msg.get_str("Foo").unwrap(), "bar");

  assert_eq!(msg.get_str("Moo"), None);
}


#[test]
fn exists() {
  let mut params = Params::new();

  params.add_str("foo", "bar").unwrap();
  assert_eq!(params.have("foo"), true);

  assert_eq!(params.have("nonexistent"), false);
}


#[test]
fn integer() {
  let mut msg = Params::new();

  msg.add_str("Num", "64").unwrap();
  assert_eq!(msg.get_int::<u16>("Num").unwrap(), 64);
}


#[test]
fn size() {
  let mut msg = Params::new();

  msg.add_param("Num", 7 as usize).unwrap();
  assert_eq!(msg.get_int::<usize>("Num").unwrap(), 7);
}


#[test]
fn intoparams() {
  let mut msg = Params::new();

  msg.add_str("Foo", "bar").unwrap();
  assert_eq!(msg.get_str("Foo").unwrap(), "bar");
  assert_eq!(msg.get_str("Moo"), None);

  let hm = msg.into_inner();
  let kv = hm.get_key_value("Foo");
  if let Some((_k, v)) = kv {
    assert_eq!(v, "bar");
  }
}


#[test]
fn display() {
  let mut params = Params::new();

  params.add_str("foo", "bar").unwrap();
  let s = format!("{}", params);
  assert_eq!(s, "{foo=bar}");
}


#[test]
fn ser_size() {
  let mut params = Params::new();

  params.add_str("foo", "bar").unwrap();
  params.add_str("moo", "cow").unwrap();

  let sz = params.calc_buf_size();

  assert_eq!(sz, 8 + 8 + 1);
}


#[test]
fn def_int() {
  let params = Params::new();

  let num = params.get_int_def::<u32>("nonexistent", 42).unwrap();

  assert_eq!(num, 42);
}


#[test]
fn broken_key() {
  let mut param = Params::new();
  assert_eq!(
    param.add_str("hell o", "world"),
    Err(Error::BadFormat("Invalid key character".to_string()))
  );
}


#[test]
fn empty_key() {
  let mut param = Params::new();
  assert_eq!(
    param.add_str("", "world"),
    Err(Error::BadFormat("Empty or broken key".to_string()))
  );
}


// vim: set ft=rust et sw=2 ts=2 sts=2 cinoptions=2 tw=79 :
