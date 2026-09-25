//! # Emailler: Email Library for Rust
//! **emailler** can send / receive emails quickly and easily.
//! Emailler brings **great performance, lightweight binary**
//! and **easy-to-use APIs** in your daily development.
//! Hope this crate can speed up your coding. Enjoy! :D
//!
//! # Usage Example
//! ```no_run
//! use emailler::{Result, Email};
//! fn main() -> Result<()> {
//!     let mut email = Email::new();
//!     email.from = "a@example.com";
//!     email.to = "b@example.com";
//!     email.smtp_server = "smtp.example.com";
//!     email.subject = "Email Subject";
//!     email.body = "Hello, world!";
//!     let resp = email.send()?;
//!     println!("Response: {resp:?}");
//! }
//! ```

pub(crate) mod errors;
pub(crate) mod receive;
pub(crate) mod send;

pub use errors::{EmailError, Result};
pub use receive::receive_emails;
pub use send::Email;
