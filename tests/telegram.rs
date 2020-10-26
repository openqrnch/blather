use blather::{Error, Telegram};

#[test]
fn simple() {
  let mut msg = Telegram::new();

  msg.set_topic("SomeTopic").unwrap();
  assert_eq!(msg.get_topic().unwrap(), "SomeTopic");

  msg.add_str("Foo", "bar").unwrap();
  assert_eq!(msg.get_str("Foo").unwrap(), "bar");

  assert_eq!(msg.get_str("Moo"), None);
}


#[test]
fn exist() {
  let mut tg = Telegram::new();

  tg.add_str("foo", "bar").unwrap();
  assert_eq!(tg.have_param("foo"), true);

  assert_eq!(tg.have_param("nonexistent"), false);
}


#[test]
fn integer() {
  let mut msg = Telegram::new();

  msg.set_topic("SomeTopic").unwrap();
  assert_eq!(msg.get_topic().unwrap(), "SomeTopic");

  msg.add_str("Num", "64").unwrap();
  assert_eq!(msg.get_int::<u16>("Num").unwrap(), 64);
}


#[test]
fn size() {
  let mut msg = Telegram::new();

  msg.add_param("Num", 7 as usize).unwrap();
  assert_eq!(msg.get_int::<usize>("Num").unwrap(), 7);
}


#[test]
fn intoparams() {
  let mut msg = Telegram::new();

  msg.add_str("Foo", "bar").unwrap();
  assert_eq!(msg.get_str("Foo").unwrap(), "bar");
  assert_eq!(msg.get_str("Moo"), None);

  let params = msg.into_params();
  let val = params.get_str("Foo");
  assert_eq!(val.unwrap(), "bar");
}


#[test]
fn display() {
  let mut tg = Telegram::new_topic("hello").unwrap();

  tg.add_param("foo", "bar").unwrap();
  let s = format!("{}", tg);
  assert_eq!(s, "hello:{foo=bar}");
}


#[test]
fn ser_size() {
  let mut tg = Telegram::new_topic("hello").unwrap();

  tg.add_str("foo", "bar").unwrap();
  tg.add_str("moo", "cow").unwrap();

  let sz = tg.calc_buf_size();

  assert_eq!(sz, 6 + 8 + 8 + 1);
}

#[test]
fn def_int() {
  let mut tg = Telegram::new();

  tg.add_str("Num", "11").unwrap();
  assert_eq!(tg.get_int_def::<u16>("Num", 17).unwrap(), 11);

  let num = tg.get_int_def::<u32>("nonexistent", 42).unwrap();

  assert_eq!(num, 42);
}


#[test]
fn bad_topic_leading() {
  let mut tg = Telegram::new();
  assert_eq!(
    tg.set_topic(" SomeTopic"),
    Err(Error::BadFormat(
      "Invalid leading topic character".to_string()
    ))
  );
}


#[test]
fn bad_topic() {
  let mut tg = Telegram::new();
  assert_eq!(
    tg.set_topic("Some Topic"),
    Err(Error::BadFormat("Invalid topic character".to_string()))
  );
}


// vim: set ft=rust et sw=2 ts=2 sts=2 cinoptions=2 tw=79 :
