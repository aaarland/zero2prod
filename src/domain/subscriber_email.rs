use validator::validate_email;

#[derive(Debug)]
pub struct SubscriberEmail(String);

impl SubscriberEmail {
    pub fn parse(s: String) -> Result<SubscriberEmail, String> {
        if validate_email(&s) {
            Ok(Self(s))
        } else {
            Err(format!("{} is not a valid subscriber email", s))
        }
    }
}

impl AsRef<str> for SubscriberEmail {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

#[cfg(test)]
mod tests {
    use crate::domain::SubscriberEmail;
    use claim::{assert_err, assert_ok};
    use fake::faker::internet::en::SafeEmail;
    use fake::Fake;

    #[test]
    fn a_valid_email_is_parsed_successfully() {
        let email = SafeEmail().fake();
        assert_ok!(SubscriberEmail::parse(email));
    }

    #[test]
    fn empty_string_is_rejected() {
        assert_err!(SubscriberEmail::parse("".to_string()));
    }

    #[test]
    fn email_missing_at_symbol_is_rejected() {
        assert_err!(SubscriberEmail::parse("test.com".to_string()));
    }

    #[test]
    fn email_missing_subject_is_rejectd() {
        assert_err!(SubscriberEmail::parse("@test.com".to_string()));
    }
}
