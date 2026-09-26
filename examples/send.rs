use emailler::Email;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut email = Email::new();
    email.from = "you@gmail.com".to_string();
    email.to = "friend@example.com".to_string();
    email.smtp_server = "smtp.gmail.com".to_string();
    email.subject = "Hello from Rust".to_string();
    email.body = "This email was sent with emailler.".to_string();

    let response = email.send("your-authentication-code")?;
    println!("Mail sent successfully: {:?}", response);
    Ok(())
}
