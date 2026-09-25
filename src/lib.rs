//! # Emailler: Email Library for Rust
//! **emailler** can send / receive emails quickly and easily.
//! Emailler brings **great performance, lightweight binary**
//! and **easy-to-use APIs** in your daily development.
//! Hope this crate can speed up your coding. Enjoy! :D
//!
//! # Usage Example
//! ```no_run
//! use emailler::Email;
//!
//! fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     let mut email = Email::new();
//!     email.from = "you@gmail.com".to_string();
//!     email.to = "friend@example.com".to_string();
//!     email.smtp_server = "smtp.gmail.com".to_string();
//!     email.subject = "Hello from Rust".to_string();
//!     email.body = "This email was sent with emailler.".to_string();
//!
//!     let response = email.send("your-app-password")?;
//!     println!("Mail sent successfully: {:?}", response);
//!     Ok(())
//! }
//! ```

pub(crate) mod errors;
pub(crate) mod receive;
pub(crate) mod send;

pub use errors::{EmailError, Result};
pub use receive::receive_emails;
pub use send::Email;
