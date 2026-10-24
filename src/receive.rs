//! Module for email receiving.

use crate::config::ImapConfig;
use crate::errors::{EmailError, Result};
use mail_parser::{
    Address::{self, *},
    MessageParser, PartType,
};
use native_tls::TlsConnector;

/// Email receipt structure, carrying extracted email metadata and content.
/// # Examples
/// ```no_run
#[doc = include_str!("../examples/receive.rs")]
/// ```
#[derive(Debug)]
pub struct EmailReceipt {
    /// Sender list of this email.
    pub senders: Vec<String>,
    /// Recipient list of this email.
    pub receivers: Vec<String>,
    /// Email subject line.
    pub subject: String,
    /// Plain text body content.
    pub body: String,
    /// HTML formatted body content.
    pub html_body: String,
}

impl EmailReceipt {
    /// Create an empty [`EmailReceipt`].
    pub fn new() -> Self {
        Self {
            senders: Vec::new(),
            receivers: Vec::new(),
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

/// Convert mail_parser Address tree into human-readable address strings.
/// Handles single address lists and grouped address lists.
fn parse_emails(addr_obj: Option<&Address>) -> Vec<String> {
    match addr_obj {
        Some(List(addrs)) => addrs
            .iter()
            .map(|addr| {
                let name = addr
                    .name
                    .as_ref()
                    .map(|s| s.as_ref())
                    .unwrap_or("(No name)");
                let email = addr
                    .address
                    .as_ref()
                    .map(|s| s.as_ref())
                    .unwrap_or("(No address)");
                format!("{name} <{email}>")
            })
            .collect(),
        Some(Group(groups)) => groups
            .iter()
            .map(|g| {
                let group_name = g
                    .name
                    .as_ref()
                    .map(|s| s.as_ref())
                    .unwrap_or("(No group name)");
                let members: Vec<_> = g
                    .addresses
                    .iter()
                    .map(|addr| {
                        let name = addr
                            .name
                            .as_ref()
                            .map(|s| s.as_ref())
                            .unwrap_or("(No name)");
                        let email = addr
                            .address
                            .as_ref()
                            .map(|s| s.as_ref())
                            .unwrap_or("(No address)");
                        format!("{name} <{email}>")
                    })
                    .collect();
                format!("{group_name}: {}", members.join(", "))
            })
            .collect(),
        None => Vec::new(),
    }
}

/// Fetch all unread messages from the specified IMAP folder using UID semantics.
/// Prefer this helper instead of raw sequence numbers, as UIDs are stable across folder changes.
fn fetch_unread(
    addr: &str,
    auth_code: &str,
    cfg: &ImapConfig,
    folder: &str,
    mark_as_read: bool,
    skip_errors: bool,
) -> Result<Vec<EmailReceipt>> {
    use crate::utils::check_email;

    if !check_email(addr) {
        return Err(EmailError::InvalidField("Address invalid".to_string()));
    }

    if cfg.use_starttls && cfg.port == 993 {
        return Err(EmailError::ImapConfig(
            "Port 993 uses Implicit TLS, set use_starttls=false".to_string(),
        ));
    }
    if !cfg.use_starttls && cfg.port == 143 {
        return Err(EmailError::ImapConfig(
            "Port 143 uses STARTTLS, set use_starttls=true".to_string(),
        ));
    }

    let tls_connector = TlsConnector::new()?;
    let client = if cfg.use_starttls {
        imap::connect_starttls((cfg.host.as_str(), cfg.port), &cfg.host, &tls_connector)?
    } else {
        imap::connect((cfg.host.as_str(), cfg.port), &cfg.host, &tls_connector)?
    };

    let mut session = client.login(addr, auth_code).map_err(|e| e.0)?;
    session.select(folder)?;

    let unread_uids = session.uid_search("UNSEEN")?;
    if unread_uids.is_empty() {
        session.logout()?;
        return Ok(Vec::new());
    }

    const BATCH_SIZE: usize = 50;
    // Materialize UidSet into Vec first
    let mut uid_list: Vec<u32> = unread_uids.into_iter().collect();
    // Sort UIDs, many IMAP servers prefer ordered UID sets
    uid_list.sort_unstable();

    let mut receipts = Vec::new();

    // Manual chunk iterator
    let mut idx = 0;
    while idx < uid_list.len() {
        let end = (idx + BATCH_SIZE).min(uid_list.len());
        let chunk = &uid_list[idx..end];
        idx = end;

        let uid_set = chunk
            .iter()
            .map(|uid| uid.to_string())
            .collect::<Vec<_>>()
            .join(",");

        let fetched = session.uid_fetch(&uid_set, "RFC822")?;
        for item in fetched.iter() {
            let Some(raw) = item.body() else {
                continue;
            };

            let Some(msg) = MessageParser::default().parse(raw) else {
                if skip_errors {
                    continue;
                } else {
                    return Err(EmailError::EmailParse(
                        "Email message parsing failed".to_string(),
                    ));
                }
            };

            let mut receipt = EmailReceipt::new();
            receipt.subject = msg.subject().unwrap_or("(No subject)").to_string();
            receipt.body = msg
                .text_bodies()
                .flat_map(|part| match &part.body {
                    PartType::Text(s) => Some(s.as_ref()),
                    _ => None,
                })
                .collect();
            receipt.html_body = msg
                .html_bodies()
                .flat_map(|part| match &part.body {
                    PartType::Html(s) => Some(s.as_ref()),
                    _ => None,
                })
                .collect();
            receipt.senders = parse_emails(msg.from());
            receipt.receivers = parse_emails(msg.to());

            receipts.push(receipt);
        }

        if mark_as_read {
            session.uid_store(&uid_set, "+FLAGS \\Seen")?;
        }
    }

    session.logout()?;
    Ok(receipts)
}

/// Receive all unread emails from INBOX.
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
    fetch_unread(addr, auth_code, cfg, "INBOX", mark_as_read, skip_errors)
}

/// Receive all unread spam/junk messages.
/// Folder name varies across providers:
/// - QQ / 163 / Outlook: `"Junk"`
/// - Gmail: `"[Gmail]/Spam"`
/// - Dovecot default: `"Spam"`
/// # Examples
/// ```no_run
#[doc = include_str!("../examples/receive_spam.rs")]
/// ```
pub fn receive_spam(
    addr: &str,
    auth_code: &str,
    cfg: &ImapConfig,
    folder: &str,
    mark_as_read: bool,
    skip_errors: bool,
) -> Result<Vec<EmailReceipt>> {
    fetch_unread(addr, auth_code, cfg, folder, mark_as_read, skip_errors)
}

#[cfg(test)]
mod tests {
    use super::EmailReceipt;
    use super::parse_emails;
    use mail_parser::{Addr, Address, Group};
    use std::borrow::Cow::Borrowed;

    #[test]
    fn test_email_receipt_construct() {
        let receipt = EmailReceipt::new();
        assert!(receipt.body.is_empty());
        assert!(receipt.receivers.is_empty());
        assert!(receipt.senders.is_empty());
        assert!(receipt.subject.is_empty());
    }

    #[test]
    fn test_email_receipt_default() {
        let opt: Option<EmailReceipt> = None;
        let receipt = opt.unwrap_or_default();
        assert!(receipt.body.is_empty());
        assert!(receipt.receivers.is_empty());
        assert!(receipt.senders.is_empty());
        assert!(receipt.subject.is_empty());
    }

    #[test]
    fn test_email_parse() {
        type OptRefAddr<'a> = Option<&'a Address<'a>>;

        let addr = Addr {
            name: Some(Borrowed("Bob")),
            address: Some(Borrowed("bob@example.com")),
        };
        let group = Group {
            name: Some(Borrowed("TestGroup")),
            addresses: vec![addr.clone()],
        };

        let case_empty_list: OptRefAddr = Some(&Address::List(vec![]));
        let case_none: OptRefAddr = None;
        let case_single: OptRefAddr = Some(&Address::List(vec![addr]));
        let case_group: OptRefAddr = Some(&Address::Group(vec![group]));
        let case_empty_group: OptRefAddr = Some(&Address::Group(vec![]));

        assert!(parse_emails(case_empty_list).is_empty());
        assert!(parse_emails(case_none).is_empty());
        assert_eq!(parse_emails(case_single).len(), 1);
        assert_eq!(parse_emails(case_single)[0], "Bob <bob@example.com>");
        assert_eq!(parse_emails(case_group).len(), 1);
        assert_eq!(
            parse_emails(case_group)[0],
            "TestGroup: Bob <bob@example.com>"
        );
        assert!(parse_emails(case_empty_group).is_empty());
    }
}
