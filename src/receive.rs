//! Module for email receiving.

use crate::{
    errors::{EmailError, Result},
    send::Email,
};
use mail_parser::{
    Address::{self, *},
    MessageParser,
};
use native_tls::TlsConnector;

fn parse_emails(addr_obj: Option<&Address>) -> String {
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
            result.join(", ")
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
            result.join("; ")
        }
        None => String::from("No senders"),
    }
}

/// Receive all unread emails, and mark them as read.
/// # Examples
/// ```no_run
/// ```
pub fn receive_emails(
    addr: &str,
    auth_code: &str,
    smtp_server: &str,
    mark_as_read: bool,
) -> Result<Vec<Email>> {
    // Simple validation
    if addr.is_empty() {
        Err(EmailError::InvalidField(String::from(
            "Address cannot be empty",
        )))?;
    }

    let tls_conn = TlsConnector::new()?;
    let client = imap::connect((smtp_server, 993), smtp_server, &tls_conn)?;
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

    let mut res: Vec<Email> = Vec::new();
    for item in fetched.iter() {
        let Some(raw) = item.body() else {
            // No RFC822 body, skip
            continue;
        };
        let Some(msg) = MessageParser::default().parse(raw) else {
            return Err(EmailError::EmailParse(String::from(
                "Email message parsing failed",
            )));
        };
        let mut email = Email::new();

        email.subject = msg.subject().unwrap_or("(No subjects)").to_string();
        email.body = msg
            .text_body
            .iter()
            .filter_map(|c| char::from_u32(*c))
            .collect();
        email.smtp_server = smtp_server.to_string();
        email.from = parse_emails(msg.from());
        email.to = parse_emails(msg.to());

        res.push(Email::new());
    }

    if mark_as_read {
        // Mark as read.
        session.store(&seq_str, "+FLAGS \\Seen")?;
    }

    session.logout()?;
    Ok(res)
}
