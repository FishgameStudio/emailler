use emailler::receive_emails;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let emails = receive_emails(
        "you@example.com",
        "your-authentication-code",
        "smtp.example.com",
        true,
    )?;
    for email in &emails {
        println!("------");
        println!("Senders: {}", email.senders.join(", "));
        println!("Receivers: {}", email.receivers.join(", "));
        println!("Subject: {}", email.subject);
        println!("Body:\n{}", email.body);
    }
    Ok(())
}
