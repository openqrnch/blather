//! A library used to represent a line-based protocol largely based on
//! key/value pairs.
//!
//! A _telegram_, represented by the Telegram struct, is an entity that
//! consists of a mandatory "topic" and zero or more key/value parameters.
//!
//! A set of "parameters", represented by the Params struct, is a set of
//! key/value pairs.
//!
//! blather handles transmission usign tokio-util's Framed framework.

pub mod codec;
mod err;
mod kvlines;
mod params;
mod telegram;
mod validators;

pub use codec::Codec;
pub use err::Error;
pub use kvlines::{KVLines, KeyValue};
pub use params::Params;
pub use telegram::Telegram;

// vim: set ft=rust et sw=2 ts=2 sts=2 cinoptions=2 tw=79 :
