//! # Emailler: Email Library for Rust
//! **emailler** can send / receive emails quickly and easily.
//! Emailler brings **great performance, lightweight binary**
//! and **easy-to-use APIs** in your daily development.
//! Hope this crate can speed up your coding. Enjoy! :D
//!
//! # Usage Example
//! ```no_run
#![doc = include_str!("../examples/send.rs")]
//! ```
//!
//! ```no_run
#![doc = include_str!("../examples/receive.rs")]
//! ```

pub(crate) mod errors;
pub(crate) mod receive;
pub(crate) mod send;

pub use errors::{EmailError, Result};
pub use receive::receive_emails;
pub use send::Email;
