//! A library used to represent a line-based protocol largely based on
//! key/value pairs.
//!
//! A "telegram", represented by the Telegram struct, is an entity that
//! consists of a mandatory "topic" and zero or more key/value parameters.
//!
//! A set of "parameters", represented by the Params struct, is a set of
//! key/value pairs.
//!
//! blather does not handle transmission; it only represents the Telegram
//! and Params buffers and provides methods to serialize them.

mod err;
mod telegram;
mod params;
mod validators;

pub use telegram::Telegram;
pub use params::Params;
pub use err::Error;

// vim: set ft=rust et sw=2 ts=2 sts=2 cinoptions=2 tw=79 :
