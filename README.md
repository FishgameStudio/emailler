# emailler

[![Crates.io](https://img.shields.io/crates/v/emailler.svg)](https://crates.io/crates/emailler)
[![Downloads](https://img.shields.io/crates/d/emailler.svg)](https://crates.io/crates/emailler)
[![Docs.rs](https://img.shields.io/badge/docs.rs-emailler-blue&logo=rust)](https://docs.rs/emailler)
[![Stars](https://img.shields.io/github/stars/FishgameStudio/emailler)](https://github.com/FishgameStudio/emailler/stargazers)
[![Open Issues](https://img.shields.io/github/issues/FishgameStudio/emailler)](https://github.com/FishgameStudio/emailler/issues)
[![Open PRs](https://img.shields.io/github/issues-pr/FishgameStudio/emailler)](https://github.com/FishgameStudio/emailler/pulls)
[![License: MIT](https://img.shields.io/badge/license-MIT-green)](https://github.com/FishgameStudio/emailler/blob/main/LICENSE)

> Send emails in Rust with a smile.

Emailler is a lightweight Rust crate for sending simple SMTP emails without the usual boilerplate. It is designed for people who want a straightforward way to deliver notifications, alerts, and transactional messages from their Rust applications.

## Why emailler?

If you are building a Rust service and need to send mail quickly, emailler gives you a direct path to the job:

- A simple email model with clear fields
- SMTP-based sending with a minimal API
- Easy setup for notifications and alerts
- Clean integration into small and medium Rust projects

This crate is not trying to replace a full email platform. Instead, it focuses on the essentials: creating an email, setting the sender and recipient, connecting to an SMTP server, and sending it reliably.

## 📦 Installation

Add this to your `Cargo.toml` file:

```toml
[dependencies]
emailler = "0.1.1"
```

Or use cargo to add it:

```sh
cargo add emailler
```

## 🚀 Quick start

```rust
use emailler::{Email, SmtpConfig};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut email = Email::new();
    email.from = "you@gmail.com".into();
    email.to = "friend@example.com".into();
    email.subject = "Hello from Rust".into();
    email.body = "This email was sent with emailler.".into();
    email.html_body = "This email was sent <strong>with emailler</strong>.".into();

    let smtp_config = SmtpConfig::new("smtp.example.com");

    let response = email.send("your-authentication-code", &smtp_config)?;
    println!("Mail sent successfully: {:?}", response);
    Ok(())
}
```

This example creates a new email, fills in the required fields, and sends it through the configured SMTP server. In real applications, you would normally load the sender address, SMTP host, and password from environment variables or a configuration file rather than hardcoding them.

## Notes

Before sending mail through services such as Gmail, Outlook, or other providers, make sure you are using the correct SMTP settings and credentials for that provider. Many email services require an application-specific password or special configuration for SMTP access.

## License

This project is licensed under the MIT license.
