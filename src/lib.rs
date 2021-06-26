//! A protocol and a communication library for a mostly line-based key/value
//! pair protocol.
//!
//! # Communication buffers
//! _blather_ defines a few buffers which it uses to send and receive
//! information over its communication module.
//!
//! ## Telegrams
//! The most central communication buffer is [`Telegram`], which
//! consists of a _topic_ and zero or more key/value pairs, where each key must
//! be unique.
//!
//! ```
//! use blather::Telegram;
//!
//! let mut tg = Telegram::new_topic("AddUser").unwrap();
//!
//! tg.add_param("Name", "Frank Foobar");
//! tg.add_param("Job", "Secret Agent");
//! tg.add_param("Age", "42");
//!
//! assert_eq!(tg.get_topic(), Some("AddUser"));
//! assert_eq!(tg.get_str("Name").unwrap(), "Frank Foobar");
//! assert_eq!(tg.get_param::<u8>("Age").unwrap(), 42);
//! ```
//!
//! ## Params
//! These are simple key/value pairs, which can be seen as `HashMap`'s with
//! some restrictions on key names.
//!
//! ```
//! use blather::Params;
//!
//! let mut params = Params::new();
//!
//! params.add_param("Name", "Frank Foobar");
//! params.add_param("Job", "Secret Agent");
//! params.add_param("Age", "42");
//!
//! assert_eq!(params.get_str("Name").unwrap(), "Frank Foobar");
//! assert_eq!(params.get_param::<u8>("Age").unwrap(), 42);
//! ```
//!
//! A set of "parameters", represented by the Params struct, is a set of
//! key/value pairs.  They look similar to `Telegrams` because the `Telegram`'s
//! implement their key/value paris using a `Params` buffer.
//!
//! # Communication
//! blather handles transmission using tokio-util's
//! [`Framed`](tokio_util::codec::Framed) framework, by
//! implementing its own [`Codec`](codec::Codec).  It can be used to send and
//! receive the various communication buffers supported by the crate.

#![deny(missing_docs)]
#![deny(missing_crate_level_docs)]
#![deny(missing_doc_code_examples)]

pub mod codec;
mod err;
pub mod types;

pub use codec::Codec;
pub use err::Error;
pub use types::{KVLines, KeyValue, Params, Telegram};

// vim: set ft=rust et sw=2 ts=2 sts=2 cinoptions=2 tw=79 :
