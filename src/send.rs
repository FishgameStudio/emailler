//! Module for email sending.

use crate::config::SmtpConfig;
use crate::errors::{EmailError, Result};
use lettre::message::{Message, MultiPart};
use lettre::transport::smtp::authentication::Credentials;
use lettre::transport::smtp::response::Response;
use lettre::{SmtpTransport, Transport};
use zeroize::Zeroize;

fn construct_mailer<S>(sender: S, authentication_code: S, cfg: &SmtpConfig) -> Result<SmtpTransport>
where
    S: Into<String>,
{
    // Validate compatibility of port and encryption mode
    if cfg.use_starttls && cfg.port == 465 {
        return Err(EmailError::SmtpConfig(
            "Port 465 uses Implicit TLS, set use_starttls=false".into(),
        ));
    }
    if !cfg.use_starttls && cfg.port == 587 {
        return Err(EmailError::SmtpConfig(
            "Port 587 uses STARTTLS, set use_starttls=true".into(),
        ));
    }

    let creds = Credentials::new(sender.into(), authentication_code.into());

    // Use starttls_relay if specified to use STARTTLS.
    let mut builder = if cfg.use_starttls {
        SmtpTransport::starttls_relay(&cfg.host)?
    } else {
        SmtpTransport::relay(&cfg.host)?
    };

    builder = builder.port(cfg.port).credentials(creds);

    if let Some(t) = cfg.timeout {
        builder = builder.timeout(Some(t));
    }

    Ok(builder.build())
}

/// Structure to store email informations.
///
/// To change / set up these fields, you need to set it as mutable.
/// ```
/// # use emailler::Email;
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
        }
    }
    /// Send this email to the specified receiver.
    /// The `auth_code` will be zeroized after usage.
    /// # Examples
    /// ```no_run
    #[doc = include_str!("../examples/send.rs")]
    /// ```
    pub fn send<S>(&self, auth_code: S, cfg: &SmtpConfig) -> Result<Response>
    where
        S: Into<String>,
    {
        let mut auth_code = auth_code.into();

        // Simple validation
        #[inline]
        fn valid_addr(email: &str, msg: &str) -> Result<()> {
            use crate::utils::check_email;
            if check_email(email) {
                Err(EmailError::InvalidField(String::from(msg)))?;
            }
            Ok(())
        }
        #[inline]
        fn non_empty(s: &str, msg: &str) -> Result<()> {
            if s.is_empty() {
                Err(EmailError::InvalidField(String::from(msg)))?;
            }
            Ok(())
        }
        valid_addr(&self.from, "Address of sender invalid")?;
        valid_addr(&self.to, "Address of receiver invalid")?;
        non_empty(&cfg.host, "SMTP server domain not specified")?;

        let mailer = construct_mailer(&self.from, &auth_code, cfg)?;
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
    use super::{Email, SmtpConfig, construct_mailer};

    #[test]
    fn construct() {
        let mailer = construct_mailer(
            "xxx@example.com",
            "xxx-auth-code",
            &SmtpConfig::new("smtp.xxx.com"),
        )
        .unwrap();
        mailer
            .test_connection()
            .expect_err("Connection should fail"); // Should fail
    }

    #[test]
    fn email() {
        let email = Email::new();
        assert!(email.body.is_empty());
        assert!(email.subject.is_empty());
        assert!(email.from.is_empty());
        assert!(email.to.is_empty());

        email
            .send("xxx-auth-code", &SmtpConfig::new("smtp.xxx.example"))
            .expect_err("Should fail"); // Should fail
    }

    #[test]
    fn email_default() {
        let opt: Option<Email> = None;
        let email = opt.unwrap_or_default();
        assert!(email.body.is_empty());
        assert!(email.subject.is_empty());
        assert!(email.from.is_empty());
        assert!(email.to.is_empty());
    }
}
