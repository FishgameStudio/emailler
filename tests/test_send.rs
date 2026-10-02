use emailler::{Email, SmtpConfig};

#[test]
fn test_email_validation() {
    let e = Email::new();

    const AUTH_CODE: &str = "illegal-auth-code";
    const HOST: &str = "illegal-host-name 111 111";
    let cfg = &SmtpConfig::new(HOST);

    e.send(AUTH_CODE, cfg).expect_err("Should fail");
    e.send_to(AUTH_CODE, cfg, "illegal-email-addr")
        .expect_err("Should fail");
}
