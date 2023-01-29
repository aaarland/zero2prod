use unicode_segmentation::UnicodeSegmentation;
#[derive(Debug)]
pub struct SubscriberName(String);

impl AsRef<str> for SubscriberName {
    fn as_ref(&self) -> &str {
        &self.0
    }
}
impl SubscriberName {
    pub fn parse(s: String) -> Result<SubscriberName, String> {
        let is_empty_or_whitespace = s.trim().is_empty();
        let is_too_long = s.graphemes(true).count() > 256;
        let forbidden_characters = [
            '<', '>', '"', '`', '(', ')', '{', '}', '\\', ';', '/', '[', ']',
        ];
        let contains_forbidden_characters = s.chars().any(|c| forbidden_characters.contains(&c));
        if is_empty_or_whitespace || is_too_long || contains_forbidden_characters {
            Err(format!("{} is not a valid subscriber name ", s))
        } else {
            Ok(Self(s))
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::domain::SubscriberName;
    use claim::{assert_err, assert_ok};

    #[test]
    fn a_256_grapheme_long_name_is_valid() {
        let name = "å".repeat(256);
        assert_ok!(SubscriberName::parse(name));
    }

    #[test]
    fn a_name_longer_than_256_graphemes_is_invalid() {
        let name = "å".repeat(257);
        assert_err!(SubscriberName::parse(name));
    }

    #[test]
    fn empty_string_is_rejected() {
        assert_err!(SubscriberName::parse("".to_string()));
    }

    #[test]
    fn names_containing_invalid_characters_are_rejected() {
        let invalid_characters = [
            '<', '>', '"', '`', '(', ')', '{', '}', '\\', ';', '/', '[', ']',
        ];
        for c in invalid_characters {
            let name = format!("name_with_{}", c);
            assert_err!(SubscriberName::parse(name));
        }
    }

    #[test]
    fn a_valid_name_is_pares_successfully() {
        let name = "John Doe".to_string();
        assert_ok!(SubscriberName::parse(name));
    }
}
