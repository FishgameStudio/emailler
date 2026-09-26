//! Commonly used utilities.

use lettre::message::Mailbox;

/// Returns whether the email is valid.
#[inline]
pub fn check_email<S>(email: S) -> bool
where
    S: Into<String>,
{
    let email = email.into();
    email.parse::<Mailbox>().is_ok()
}

#[cfg(test)]
mod tests {
    use super::check_email;

    #[test]
    fn test_check_email() {
        assert!(check_email("a@a.com"));
        assert!(check_email("user+tag@example.com"));
        assert!(check_email("user.name@a.b.c.com"));
        assert!(check_email("123ABC@abc.com"));
        assert!(check_email("!#$%&'*+-/=?^@aaa-aaa.com"));
        assert!(check_email("Bob <aaaa@bbbb.org>"));
        assert!(check_email("Alice Smith <alice@example.com>"));

        assert!(!check_email("")); // empty
        assert!(!check_email("alice@"));
        assert!(!check_email("@example.com"));
        assert!(!check_email("xxx@@xxx.com"));
        assert!(!check_email("alice..smith@example.com"));
        assert!(!check_email(".aa@a.com"));
        assert!(!check_email("aa.@a.com"));
        assert!(!check_email("alice@.com"));
        assert!(!check_email("alice@-a.com"));
        assert!(!check_email("alice"));
    }
}
