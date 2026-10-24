use emailler::{ImapConfig, receive_spam};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let imap_config = ImapConfig::new("imap.example.com");
    let emails = receive_spam(
        "you@example.com",
        "your-authentication-code",
        &imap_config,
        "Junk",
        true,
        true,
    )?;
    println!("Spam/Junk emails:");
    for email in &emails {
        println!("------");
        println!("Senders: {}", email.senders.join(", "));
        println!("Receivers: {}", email.receivers.join(", "));
        println!("Subject: {}", email.subject);
        println!("Body:\n{}", email.body);
    }
    Ok(())
}
