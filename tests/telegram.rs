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


#[test]
fn ser_size() {
  let mut tg = Telegram::new_topic("hello").unwrap();

  tg.add_str("foo", "bar");
  tg.add_str("moo", "cow");

  let sz = tg.calc_buf_size();

  assert_eq!(sz, 6+8+8+1);
}

#[test]
fn def_int() {
  let mut tg = Telegram::new();

  tg.add_str("Num", "11");
  assert_eq!(tg.get_int_def::<u16>("Num", 17).unwrap(), 11);

  let num = tg.get_int_def::<u32>("nonexistent", 42).unwrap();

  assert_eq!(num, 42);
}


// vim: set ft=rust et sw=2 ts=2 sts=2 cinoptions=2 tw=79 :
