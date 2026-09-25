//! # Emailler: Email Library for Rust
//!

pub(crate) mod errors;
pub(crate) mod receive;
pub(crate) mod send;

pub use errors::{EmailError, Result};
pub use receive::receive_emails;
pub use send::Email;
