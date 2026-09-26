//! # Error Types
//! This modules included error types for email sending & receiving.

use thiserror::Error;

/// Enumeration for errors during email proccess.
/// - [`AddressParse`](EmailError::AddressParse): Errors during parsing email address.
/// - [`MessageBuild`](EmailError::MessageBuild): Errors during building email messages.
/// - [`SmtpTransport`](EmailError::SmtpTransport): Errors during creating SMTP transports.
/// - [`InvalidField`](EmailError::InvalidField): Errors when the fields are invalid.
/// - [`TlsConnectorCreation`](EmailError::TlsConnectorCreation): Errors during creating TLS connector.
/// - [`Client`](EmailError::Client): Errors during the client or the session.
/// - [`EmailParse`](EmailError::EmailParse): Errors during parsing email contents.
#[derive(Debug, Error)]
#[non_exhaustive]
pub enum EmailError {
    /// Errors during parsing email address.
    #[error("Failed to parse email address: {0}")]
    AddressParse(#[from] lettre::address::AddressError),

    /// Errors during building email messages.
    #[error("Failed to build email message: {0}")]
    MessageBuild(#[from] lettre::error::Error),

    /// Errors during creating SMTP transports.
    #[error("SMTP transport error: {0}")]
    SmtpTransport(#[from] lettre::transport::smtp::Error),

    /// Errors when the fields are invalid.
    #[error("Invalid email field: {0}")]
    InvalidField(String),

    /// Errors during creating TLS connector.
    #[error("Failed to create TLS connector: {0}")]
    TlsConnectorCreation(#[from] native_tls::Error),

    /// Errors during the client or the session.
    #[error("Client error: {0}")]
    Client(#[from] imap::Error),

    /// Errors during parsing email contents.
    #[error("Failed to parse email content: {0}")]
    EmailParse(String),

    /// Errors during processing SMTP configurations.
    #[error("SMTP config error: {0}")]
    SmtpConfig(String),

    /// Errors during processing IMAP configurations.
    #[error("IMAP config error: {0}")]
    ImapConfig(String),
}

/// Generic [`Result`](crate::errors::Result) for this crate.
pub type Result<T> = core::result::Result<T, EmailError>;
