use emailler::{Email, SmtpConfig};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut email = Email::new();
    email.from = "you@gmail.com".into();
    email.to = "friend@example.com".into();
    email.subject = "Hello from Rust".into();
    email.body = "This email was sent with emailler.".into();
    email.html_body = "This email was sent <strong>with emailler</strong>.".into();

    let smtp_config = SmtpConfig::new("smtp.example.com");

    let response = email.send("your-authentication-code", &smtp_config)?;
    println!("Mail sent successfully: {:?}", response);
    Ok(())
}
