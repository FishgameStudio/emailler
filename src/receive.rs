//! Module for email receiving.

use crate::config::ImapConfig;
use crate::errors::{EmailError, Result};
use mail_parser::{
    Address::{self, *},
    MessageParser, PartType,
};
use native_tls::TlsConnector;

/// Email receipts, without complex sending informations.
/// # Examples
/// ```no_run
#[doc = include_str!("../examples/receive.rs")]
/// ```
#[derive(Debug)]
pub struct EmailReceipt {
    /// The senders who emails you.
    pub senders: Vec<String>,
    /// The receivers, including you.
    pub receivers: Vec<String>,
    /// The subject of the email.
    pub subject: String,
    /// The text body.
    pub body: String,
    /// The HTML body.
    pub html_body: String,
}
impl EmailReceipt {
    /// Create a new [`EmailReceipt`] object.
    pub fn new() -> Self {
        Self {
            senders: vec![],
            receivers: vec![],
            subject: String::new(),
            body: String::new(),
            html_body: String::new(),
        }
    }
}
impl Default for EmailReceipt {
    fn default() -> Self {
        Self::new()
    }
}

fn parse_emails(addr_obj: Option<&Address>) -> Vec<String> {
    // Too complicated to parse!
    match addr_obj {
        Some(List(addrs)) => {
            // Multiple senders
            let mut result: Vec<String> = vec![];
            for addr in addrs {
                let name = addr
                    .name
                    .as_ref()
                    .map(|s| s.as_ref())
                    .unwrap_or("(No names)");
                let email = addr
                    .address
                    .as_ref()
                    .map(|s| s.as_ref())
                    .unwrap_or("(No emails)");
                result.push(format!("{name} <{email}>"));
            }
            result
        }
        Some(Group(groups)) => {
            // Multiple groups
            let mut result: Vec<String> = vec![];
            for g in groups {
                let g_name = g
                    .name
                    .as_ref()
                    .map(|s| s.as_ref())
                    .unwrap_or("(No group names)");
                let mut result2: Vec<String> = vec![];
                for addr in &g.addresses {
                    let name = addr
                        .name
                        .as_ref()
                        .map(|s| s.as_ref())
                        .unwrap_or("(No names)");
                    let email = addr
                        .address
                        .as_ref()
                        .map(|s| s.as_ref())
                        .unwrap_or("(No emails)");
                    result2.push(format!("{} <{}>", name, email));
                }
                result.push(format!("{g_name}:  {}", result2.join(", ")));
            }
            result
        }
        None => vec![],
    }
}

/// Receive all unread emails, and mark them as read if need.
/// # Examples
/// ```no_run
#[doc = include_str!("../examples/receive.rs")]
/// ```
pub fn receive_emails(
    addr: &str,
    auth_code: &str,
    cfg: &ImapConfig,
    mark_as_read: bool,
    skip_errors: bool,
) -> Result<Vec<EmailReceipt>> {
    // Simple validation
    use crate::utils::check_email;
    if !check_email(addr) {
        Err(EmailError::InvalidField(String::from("Address invalid")))?;
    }
    if cfg.use_starttls && cfg.port == 993 {
        return Err(EmailError::ImapConfig(
            "Port 993 uses Implicit TLS, set use_starttls=false".into(),
        ));
    }
    if !cfg.use_starttls && cfg.port == 143 {
        return Err(EmailError::ImapConfig(
            "Port 143 uses STARTTLS, set use_starttls=true".into(),
        ));
    }

    let tls_conn = TlsConnector::new()?;
    let client = if cfg.use_starttls {
        imap::connect_starttls((cfg.host.as_str(), cfg.port), &cfg.host, &tls_conn)?
    } else {
        imap::connect((cfg.host.as_str(), cfg.port), &cfg.host, &tls_conn)?
    };
    let mut session = client.login(addr, auth_code).map_err(|e| e.0)?;

    session.select("INBOX")?;
    let unread_ids = session.search("UNSEEN")?;
    if unread_ids.is_empty() {
        // No unread emails.
        session.logout()?;
        return Ok(vec![]);
    }

    let seq_str: String = unread_ids
        .iter()
        .map(|n| n.to_string())
        .collect::<Vec<_>>()
        .join(",");
    let fetched = session.fetch(&seq_str, "RFC822")?;

    let mut res: Vec<EmailReceipt> = Vec::new();
    for item in fetched.iter() {
        let Some(raw) = item.body() else {
            // No RFC822 body, skip
            continue;
        };
        let Some(msg) = MessageParser::default().parse(raw) else {
            if skip_errors {
                continue;
            } else {
                return Err(EmailError::EmailParse(String::from(
                    "Email message parsing failed",
                )));
            }
        };
        let mut email = EmailReceipt::new();

        email.subject = msg.subject().unwrap_or("(No subjects)").to_string();
        email.body = msg
            .text_bodies()
            .flat_map(|part| match &part.body {
                PartType::Text(s) => Some(s.as_ref()),
                _ => None,
            })
            .collect::<String>();

        email.html_body = msg
            .html_bodies()
            .flat_map(|part| match &part.body {
                PartType::Html(s) => Some(s.as_ref()),
                _ => None,
            })
            .collect::<String>();

        email.senders = parse_emails(msg.from());
        email.receivers = parse_emails(msg.to());

        res.push(email);
    }

    if mark_as_read {
        // Mark as read.
        session.store(&seq_str, "+FLAGS \\Seen")?;
    }

    session.logout()?;
    Ok(res)
}

#[cfg(test)]
mod tests {
    use super::EmailReceipt;
    use super::parse_emails;
    use mail_parser::{Addr, Address, Group};
    use std::borrow::Cow::Borrowed;

    #[test]
    fn email_receipt() {
        let er = EmailReceipt::new();
        assert!(er.body.is_empty());
        assert!(er.receivers.is_empty());
        assert!(er.senders.is_empty());
        assert!(er.subject.is_empty());
    }
    #[test]
    fn receipt_default() {
        let opt: Option<EmailReceipt> = None;
        let er = opt.unwrap_or_default();
        assert!(er.body.is_empty());
        assert!(er.receivers.is_empty());
        assert!(er.senders.is_empty());
        assert!(er.subject.is_empty());
    }
    #[test]
    fn email_parse() {
        type OptRefAddr<'a> = Option<&'a Address<'a>>;

        let addr = Addr {
            name: Some(Borrowed("Bob")),
            address: Some(Borrowed("bob@example.com")),
        };
        let group = Group {
            name: Some(Borrowed("TestGroup")),
            addresses: vec![addr.clone()],
        };

        let addr1: OptRefAddr = Some(&Address::List(vec![]));
        let addr2: OptRefAddr = None;
        let addr3: OptRefAddr = Some(&Address::List(vec![addr]));
        let addr4: OptRefAddr = Some(&Address::Group(vec![group]));
        let addr5: OptRefAddr = Some(&Address::Group(vec![]));
        assert!(parse_emails(addr1).is_empty());
        assert!(parse_emails(addr2).is_empty());
        assert_eq!(parse_emails(addr3).len(), 1);
        assert_eq!(parse_emails(addr3)[0], "Bob <bob@example.com>");
        assert_eq!(parse_emails(addr4).len(), 1);
        assert_eq!(parse_emails(addr4)[0], "TestGroup:  Bob <bob@example.com>");
        assert!(parse_emails(addr5).is_empty());
    }
}
