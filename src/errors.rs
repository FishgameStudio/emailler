//! # Error Types
//! This modules included error types for email sending.

use thiserror::Error;

#[derive(Debug, Error)]
pub enum EmailError {
    #[error("Failed to parse email address: {0}")]
    AddressParse(#[from] lettre::address::AddressError),

    #[error("Failed to build email message: {0}")]
    MessageBuild(#[from] lettre::error::Error),

    #[error("SMTP transport error: {0}")]
    SmtpTransport(#[from] lettre::transport::smtp::Error),

    #[error("Invalid email field: {0}")]
    InvalidField(String),

    #[error("Failed to create TLS connector: {0}")]
    TlsConnectorCreation(#[from] native_tls::Error),

    #[error("Client error: {0}")]
    Client(#[from] imap::Error),

    #[error("Failed to parse email: {0}")]
    EmailParse(String),
}

/// Generic [`Result`](crate::errors::Result) for this crate.
pub type Result<T> = core::result::Result<T, EmailError>;
