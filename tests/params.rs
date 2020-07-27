use blather::Params;

#[test]
fn string() {
  let mut msg = Params::new();

  msg.add_str("Foo", "bar");
  assert_eq!(msg.get_str("Foo").unwrap(), "bar");

  assert_eq!(msg.get_str("Moo"), None);
}


#[test]
fn integer() {
  let mut msg = Params::new();

  msg.add_str("Num", "64");
  assert_eq!(msg.get_int::<u16>("Num").unwrap(), 64);
}


#[test]
fn size() {
  let mut msg = Params::new();

  msg.add_param("Num", 7 as usize);
  assert_eq!(msg.get_int::<usize>("Num").unwrap(), 7);
}


#[test]
fn intoparams() {
  let mut msg = Params::new();

  msg.add_str("Foo", "bar");
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

  params.add_str("foo", "bar");
  let s = format!("{}", params);
  assert_eq!(s, "{foo=bar}");
}

#[test]
fn ser_size() {
  let mut params = Params::new();

  params.add_str("foo", "bar");
  params.add_str("moo", "cow");

  let sz = params.calc_buf_size();

  assert_eq!(sz, 8+8+1);
}


// vim: set ft=rust et sw=2 ts=2 sts=2 cinoptions=2 tw=79 :
