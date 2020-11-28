use std::collections::HashSet;

use blather::Params;

#[test]
fn strvec_empty() {
  let mut params = Params::new();
  params.add_str("hello", "").unwrap();

  let sv = params.get_strvec("hello").unwrap();
  assert_eq!(sv.len(), 0);
}


#[test]
fn strvec_single() {
  let mut params = Params::new();
  params.add_str("hello", "foo").unwrap();

  let sv = params.get_strvec("hello").unwrap();
  assert_eq!(sv.len(), 1);
  assert_eq!(sv[0], "foo");
}


#[test]
fn strvec_two() {
  let mut params = Params::new();
  params.add_str("hello", "foo,bar").unwrap();

  let sv = params.get_strvec("hello").unwrap();
  assert_eq!(sv.len(), 2);
  assert_eq!(sv[0], "foo");
  assert_eq!(sv[1], "bar");
}


#[test]
fn strvec_single_add() {
  let mut params = Params::new();

  let mut sv = Vec::new();
  sv.push("foo");
  params.add_strit("hello", &sv).unwrap();


  //let v = params.get_str("hello").unwrap();
  assert_eq!(params.get_str("hello"), Some("foo"));


  let sv = params.get_strvec("hello").unwrap();
  assert_eq!(sv.len(), 1);
  assert_eq!(sv[0], "foo");
}


#[test]
fn strvec_two_add() {
  let mut params = Params::new();

  let mut sv = Vec::new();
  sv.push("foo");
  sv.push("bar");
  params.add_strit("hello", &sv).unwrap();

  assert_eq!(params.get_str("hello"), Some("foo,bar"));

  let sv = params.get_strvec("hello").unwrap();
  assert_eq!(sv.len(), 2);
  assert_eq!(sv[0], "foo");
  assert_eq!(sv[1], "bar");
}


#[test]
fn slice_two_add() {
  let mut params = Params::new();

  let slice = &["foo", "bar"];
  params.add_strit("hello", slice).unwrap();

  assert_eq!(params.get_str("hello"), Some("foo,bar"));

  let sv = params.get_strvec("hello").unwrap();
  assert_eq!(sv.len(), 2);
  assert_eq!(sv[0], "foo");
  assert_eq!(sv[1], "bar");
}


#[test]
fn add_hashset() {
  let mut params = Params::new();

  let mut hs = HashSet::new();
  hs.insert("foo");
  hs.insert("bar");

  params.add_strit("hello", hs).unwrap();

  let sv = params.get_strvec("hello").unwrap();
  assert_eq!(sv.len(), 2);

  if sv[0] == "foo" {
    assert_eq!(sv[1], "bar");
  } else {
    assert_eq!(sv[0], "bar");
    assert_eq!(sv[1], "foo");
  }
}


// vim: set ft=rust et sw=2 ts=2 sts=2 cinoptions=2 tw=79 :
