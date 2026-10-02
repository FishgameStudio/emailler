//! Commonly used utilities.

use lettre::address::AddressError;
use lettre::message::Mailbox;
use url::Host;

/// Returns whether the email is valid.
#[inline]
pub(crate) fn check_email<S>(email: S) -> bool
where
    S: Into<String>,
{
    email.into().parse::<Mailbox>().is_ok()
}

/// Get the [`Err`] value
#[inline]
pub(crate) fn check_email_err<S>(email: S) -> Result<(), AddressError>
where
    S: Into<String>,
{
    email.into().parse::<Mailbox>().map(|_| ())
}

/// Returns whether the host string is valid.
#[inline]
pub(crate) fn check_host(host: &str) -> bool {
    Host::parse(host).is_ok()
}

#[cfg(test)]
mod tests {
    use super::{check_email, check_host};

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

    #[test]
    fn test_check_host() {
        assert!(check_host("127.0.0.1"));
        assert!(check_host("123.123.123.123"));
        assert!(check_host("[::1]"));
        assert!(check_host("[fe80::aabb:ccff:fedd:eeff]"));

        assert!(!check_host("https://smtp.xxx.com")); // No HTTP(S) headers!
        assert!(!check_host("http://hello.org"));
        assert!(!check_host("")); // empty
        assert!(!check_host("a:::::"));
        assert!(!check_host("1.1"));
        assert!(!check_host("..."));
    }
}
