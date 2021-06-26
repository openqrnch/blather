use tokio_stream::StreamExt;

use tokio_test::io::Builder;

use tokio_util::codec::Framed;

use blather::{codec, Codec};

#[tokio::test]
async fn tg_followed_by_buf() {
  let mut mock = Builder::new();

  mock.read(b"hello\nlen 4\n\n1234");

  let mut frm = Framed::new(mock.build(), Codec::new());

  while let Some(o) = frm.next().await {
    let o = o.unwrap();
    if let codec::Input::Telegram(tg) = o {
      assert_eq!(tg.get_topic(), Some("hello"));
      assert_eq!(tg.get_int::<usize>("len").unwrap(), 4);
      frm.codec_mut().expect_bytesmut(4).unwrap();
      break;
    } else {
      panic!("Not a Telegram");
    }
  }

  while let Some(o) = frm.next().await {
    let o = o.unwrap();
    if let codec::Input::BytesMut(_bm) = o {
    } else {
      panic!("Not a Buf");
    }
  }
}

// vim: set ft=rust et sw=2 ts=2 sts=2 cinoptions=2 tw=79 :
