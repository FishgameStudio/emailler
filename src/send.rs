//! Module for email sending.

use crate::errors::{EmailError, Result};
use lettre::message::Message;
use lettre::transport::smtp::authentication::Credentials;
use lettre::transport::smtp::response::Response;
use lettre::{SmtpTransport, Transport};
use zeroize::Zeroize;

fn construct_mailer<S>(sender: S, authentication_code: S, smtp_server: S) -> Result<SmtpTransport>
where
    S: Into<String>,
{
    let creds = Credentials::new(sender.into(), authentication_code.into());
    Ok(SmtpTransport::starttls_relay(&smtp_server.into())?
        .port(587)
        .credentials(creds)
        .build())
}

/// Structure to store email informations.
/// # Attributes
/// - **from**: The address of the sender.
/// - **to**: The address of the receiver.
/// - **subject**: The subject of the email.
/// - **body**: The body of the email.
/// - **smtp_server**: The domain of the email, e.g. `smtp.gmail.com`.
///
/// To change / set up these fields, you need to set it as mutable.
#[derive(Debug)]
pub struct Email {
    /// The sender.
    /// For examples:
    /// ```text
    /// a@example.com
    /// Alice b@example.com
    /// ```
    pub from: String,
    /// The receiver.
    /// For examples:
    /// ```text
    /// a@example.com
    /// Bob b@example.com
    /// ```
    pub to: String,
    /// The subject of the email.
    pub subject: String,
    /// The body of the email.
    pub body: String,
    /// The SMTP server domain.
    /// For example, `smtp.gmail.com` (Gmail),
    /// `smtp.office365.com` (Outlook).
    pub smtp_server: String,
}
impl Email {
    /// Create a new [`Email`] object.
    /// # Example
    /// ```
    /// use emailler::Email;
    /// fn main() {
    ///     let email = Email::new();
    ///     assert!(email.from.is_empty());
    /// }
    /// ```
    pub fn new() -> Self {
        Self {
            from: String::new(),
            to: String::new(),
            subject: String::new(),
            body: String::new(),
            smtp_server: String::new(),
        }
    }
    /// Send this email to the specified receiver.
    /// The `auth_code` will be zeroized after usage.
    /// # Examples
    /// ```no_run
    /// fn main() -> Result<()> {
    ///     let mut email = Email::new();
    ///     email.from = "a@example.com";
    ///     email.to = "b@example.com";
    ///     email.smtp_server = "smtp.example.com";
    ///     email.subject = "Email Subject";
    ///     email.body = "Hello, world!";
    ///     let resp = email.send()?;
    ///     println!("Response: {resp:?}");
    /// }
    /// ```
    pub fn send<S>(&self, auth_code: S) -> Result<Response>
    where
        S: Into<String>,
    {
        let mut auth_code = auth_code.into();

        // Simple validation
        #[inline]
        fn non_empty(s: &str, msg: &str) -> Result<()> {
            if s.is_empty() {
                Err(EmailError::InvalidField(String::from(msg)))?;
            }
            Ok(())
        }
        non_empty(&self.from, "Address of sender cannot be empty")?;
        non_empty(&self.to, "Address of receiver cannot be empty")?;
        non_empty(&self.smtp_server, "SMTP server domain not specified")?;

        let mailer = construct_mailer(&self.from, &auth_code, &self.smtp_server)?;
        let message = Message::builder()
            .from(self.from.parse()?)
            .to(self.to.parse()?)
            .subject(&self.subject)
            .body(self.body.to_owned())?;
        let resp = mailer.send(&message)?;

        auth_code.zeroize();
        drop(auth_code);
        Ok(resp)
    }
}

impl Default for Email {
    fn default() -> Self {
        Self::new()
    }
}
