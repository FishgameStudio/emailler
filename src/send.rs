//! Module for email sending.

use crate::errors::{EmailError, Result};
use lettre::message::{Message, MultiPart};
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
///
/// To change / set up these fields, you need to set it as mutable.
/// ```
/// let mut email = Email::new();
/// // Some modifications...
/// ```
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
    /// The HTML body of the email.
    pub html_body: String,
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
            html_body: String::new(),
            smtp_server: String::new(),
        }
    }
    /// Send this email to the specified receiver.
    /// The `auth_code` will be zeroized after usage.
    /// # Examples
    /// ```no_run
    #[doc = include_str!("../examples/send.rs")]
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
            .multipart(MultiPart::alternative_plain_html(
                self.body.to_owned(),
                self.html_body.to_owned(),
            ))?;
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

#[cfg(test)]
mod tests {
    use super::{Email, construct_mailer};

    #[test]
    fn construct() {
        let mailer =
            construct_mailer("xxx@example.com", "xxx-auth-code", "smtp.example.com").unwrap();
        mailer
            .test_connection()
            .expect_err("Connection should fail"); // Should fail
    }

    #[test]
    fn email() {
        let email = Email::new();
        assert!(email.body.is_empty());
        assert!(email.subject.is_empty());
        assert!(email.smtp_server.is_empty());
        assert!(email.from.is_empty());
        assert!(email.to.is_empty());

        email.send("xxx-auth-code").expect_err("Should fail"); // Should fail
    }

    #[test]
    fn email_default() {
        let opt: Option<Email> = None;
        let email = opt.unwrap_or_default();
        assert!(email.body.is_empty());
        assert!(email.subject.is_empty());
        assert!(email.smtp_server.is_empty());
        assert!(email.from.is_empty());
        assert!(email.to.is_empty());
    }
}
