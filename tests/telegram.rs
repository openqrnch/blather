use blather::Telegram;

#[test]
fn simple() {
  let mut msg = Telegram::new();

  msg.set_topic("SomeTopic").unwrap();
  assert_eq!(msg.get_topic().unwrap(), "SomeTopic");

  msg.add_str("Foo", "bar");
  assert_eq!(msg.get_str("Foo").unwrap(), "bar");

  assert_eq!(msg.get_str("Moo"), None);
}


#[test]
fn integer() {
  let mut msg = Telegram::new();

  msg.set_topic("SomeTopic").unwrap();
  assert_eq!(msg.get_topic().unwrap(), "SomeTopic");

  msg.add_str("Num", "64");
  assert_eq!(msg.get_int::<u16>("Num").unwrap(), 64);
}


#[test]
fn size() {
  let mut msg = Telegram::new();

  msg.add_param("Num", 7 as usize);
  assert_eq!(msg.get_int::<usize>("Num").unwrap(), 7);
}


#[test]
fn intoparams() {
  let mut msg = Telegram::new();

  msg.add_str("Foo", "bar");
  assert_eq!(msg.get_str("Foo").unwrap(), "bar");
  assert_eq!(msg.get_str("Moo"), None);

  let params = msg.into_params();
  let val = params.get_str("Foo");
  assert_eq!(val.unwrap(), "bar");
}


#[test]
fn display() {
  let mut tg = Telegram::new_topic("hello").unwrap();

  tg.add_param("foo", "bar");
  let s = format!("{}", tg);
  assert_eq!(s, "hello:{foo=bar}");
}

// vim: set ft=rust et sw=2 ts=2 sts=2 cinoptions=2 tw=79 :
