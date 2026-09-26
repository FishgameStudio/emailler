//! SMTP & IMAP cofigurations.

use std::time::Duration;

/// SMTP configurations.
#[derive(Debug)]
pub struct SmtpConfig {
    /// SMTP server domain, e.g. smtp.gmail.com
    pub host: String,
    /// Port, commonly 587 (STARTTLS), 465 (SSL/TLS), 25
    pub port: u16,
    /// Whether to use STARTTLS; use SmtpTransport::relay if false (directly TLS, 465).
    pub use_starttls: bool,
    /// Timeout, seconds
    pub timeout: Option<Duration>,
}
impl SmtpConfig {
    /// Create a new [`SmtpConfig`] object.
    /// Uses `STARTTLS` by default.
    pub fn new<S>(host: S) -> Self
    where
        S: Into<String>,
    {
        Self {
            host: host.into(),
            port: 587,
            use_starttls: true,
            timeout: None,
        }
    }
    /// Update fields with given arguments.
    pub fn update<S>(&mut self, host: S, port: u16, use_starttls: bool, timeout: u64)
    where
        S: Into<String>,
    {
        let host = host.into();
        let timeout = Duration::new(timeout, 0);
        self.host = host;
        self.port = port;
        self.use_starttls = use_starttls;
        self.timeout = Some(timeout);
    }
}
impl Default for SmtpConfig {
    fn default() -> Self {
        Self::new("")
    }
}

/// IMAP configurations.
#[derive(Debug)]
pub struct ImapConfig {
    /// IMAP server domain, e.g. imap.gmail.com
    pub host: String,
    /// Port, commonly 993 (Implicit TLS), 143 (STARTTLS)
    pub port: u16,
    /// Whether to use STARTTLS; use SmtpTransport::relay if false (directly TLS, 465).
    pub use_starttls: bool,
    /// Timeout, seconds
    pub timeout: Option<Duration>,
}
impl ImapConfig {
    /// Create a new [`ImapConfig`] object.
    /// Uses `STARTTLS` by default.
    pub fn new<S>(host: S) -> Self
    where
        S: Into<String>,
    {
        Self {
            host: host.into(),
            port: 993,
            use_starttls: true,
            timeout: None,
        }
    }
    /// Update fields with given arguments.
    pub fn update<S>(&mut self, host: S, port: u16, use_starttls: bool, timeout: u64)
    where
        S: Into<String>,
    {
        let host = host.into();
        let timeout = Duration::new(timeout, 0);
        self.host = host;
        self.port = port;
        self.use_starttls = use_starttls;
        self.timeout = Some(timeout);
    }
}
impl Default for ImapConfig {
    fn default() -> Self {
        Self::new("")
    }
}

#[cfg(test)]
mod tests {
    use super::{ImapConfig, SmtpConfig};
    use std::time::Duration;

    #[test]
    fn smtp_test() {
        let mut cfg = SmtpConfig::new("smtp.xxx.com");
        assert_eq!(cfg.port, 587);
        assert!(cfg.use_starttls);
        assert_eq!(cfg.timeout, None);

        cfg.update("smtp.yyy.com", 123, false, 12345);
        assert_eq!(cfg.host, "smtp.yyy.com");
        assert_eq!(cfg.port, 123);
        assert!(!cfg.use_starttls);
        assert_eq!(cfg.timeout, Some(Duration::new(12345, 0)));

        // Default
        let opt: Option<SmtpConfig> = None;
        let cfg = opt.unwrap_or_default();
        assert_eq!(cfg.port, 587);
        assert!(cfg.use_starttls);
        assert_eq!(cfg.timeout, None);
    }

    #[test]
    fn imap_test() {
        let mut cfg = ImapConfig::new("smtp.xxx.com");
        assert_eq!(cfg.port, 587);
        assert!(cfg.use_starttls);
        assert_eq!(cfg.timeout, None);

        cfg.update("smtp.yyy.com", 123, false, 12345);
        assert_eq!(cfg.host, "smtp.yyy.com");
        assert_eq!(cfg.port, 123);
        assert!(!cfg.use_starttls);
        assert_eq!(cfg.timeout, Some(Duration::new(12345, 0)));

        // Default
        let opt: Option<ImapConfig> = None;
        let cfg = opt.unwrap_or_default();
        assert_eq!(cfg.port, 587);
        assert!(cfg.use_starttls);
        assert_eq!(cfg.timeout, None);
    }
}
