//! Collection of data types which can be sent/received using the internal
//! [`Codec`](crate::codec::Codec)

pub mod kvlines;
pub mod params;
pub mod telegram;

mod validators;

pub use kvlines::{KVLines, KeyValue};
pub use params::Params;
pub use telegram::Telegram;

// vim: set ft=rust et sw=2 ts=2 sts=2 cinoptions=2 tw=79 :
