//! # Minreq
//!
//! Simple, minimal-dependency HTTP client.  The library has a very
//! minimal API, so you'll probably know everything you need to after
//! reading a few examples.

#![deny(missing_docs)]

#[cfg(feature = "json-using-serde")]
extern crate serde;
#[cfg(feature = "json-using-serde")]
extern crate serde_json;

mod connection;
mod error;
mod http_url;

mod request;
mod response;

pub use error::*;
pub use request::*;
pub use response::*;
