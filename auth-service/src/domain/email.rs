use std::fmt::Display;
use validator::ValidateEmail;

#[derive(Debug)]
pub struct ParseEmailError {
    pub value: String,
}

impl Display for ParseEmailError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} is not a valid email", self.value)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Email(String);

impl Email {
    pub fn parse(email: &str) -> Result<Self, ParseEmailError> {
        if !email.validate_email() {
            return Err(ParseEmailError {
                value: email.to_string(),
            });
        }

        Ok(Email(email.to_string()))
    }
}

impl TryFrom<&str> for Email {
    type Error = ParseEmailError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Email::parse(value)
    }
}

impl TryFrom<String> for Email {
    type Error = ParseEmailError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        Email::parse(&value)
    }
}

impl AsRef<str> for Email {
    fn as_ref(&self) -> &str {
        self.0.as_str()
    }
}

#[cfg(test)]
mod tests {
    use crate::domain::email::Email;
    use claims::assert_err;
    use fake::Fake;
    use fake::faker::internet::en::SafeEmail;
    use rand::SeedableRng;
    use rand::rngs::StdRng;

    #[derive(Debug, Clone)]
    struct ValidEmailFixture(pub String);

    impl quickcheck::Arbitrary for ValidEmailFixture {
        fn arbitrary(g: &mut quickcheck::Gen) -> Self {
            let mut rng = StdRng::seed_from_u64(u64::arbitrary(g));
            let email = SafeEmail().fake_with_rng(&mut rng);

            Self(email)
        }
    }

    #[quickcheck_macros::quickcheck]
    fn valid_emails_are_parsed_successfully(valid_email: ValidEmailFixture) -> bool {
        Email::try_from(valid_email.0).is_ok()
    }

    #[test]
    fn empty_email_is_rejected() {
        let email = Email::try_from("");
        assert_err!(email);
    }

    #[test]
    fn email_missing_at_symbol_is_rejected() {
        let email = Email::try_from("johnexample.com");
        assert_err!(email);
    }

    #[test]
    fn email_missing_subject_is_rejected() {
        let email = Email::try_from("@example.com");
        assert_err!(email);
    }
}
