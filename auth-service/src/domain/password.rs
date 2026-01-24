use std::fmt::Display;

#[derive(Debug)]
pub enum ParsePasswordError {
    Short,
    MissingSpecialSymbol,
    MissingNumber,
}

impl Display for ParsePasswordError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ParsePasswordError::Short => write!(
                f,
                "password is too short, it must contain at least 8 characters"
            ),
            ParsePasswordError::MissingSpecialSymbol => write!(
                f,
                "password must contain at least one special character: {}",
                SPECIAL_CHARACTERS.iter().collect::<String>()
            ),
            ParsePasswordError::MissingNumber => {
                write!(f, "password must contain at least one number")
            }
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Password(String);

const SPECIAL_CHARACTERS: [char; 32] = [
    '!', '”', '#', '$', '%', '&', '’', '(', ')', '*', '+', ',', '-', '.', '/', ':', ';', '<', '=',
    '>', '?', '@', '[', '\\', ']', '^', '_', '`', '{', '|', '}', '~',
];

const NUMBERS: [char; 10] = ['0', '1', '2', '3', '4', '5', '6', '7', '8', '9'];

const MIN_PASSWORD_LENGTH: usize = 8;

impl Password {
    pub fn parse(password: &str) -> Result<Self, ParsePasswordError> {
        if password.len() < MIN_PASSWORD_LENGTH {
            return Err(ParsePasswordError::Short);
        }

        if !password.contains(SPECIAL_CHARACTERS) {
            return Err(ParsePasswordError::MissingSpecialSymbol);
        }

        if !password.contains(NUMBERS) {
            return Err(ParsePasswordError::MissingNumber);
        }

        Ok(Password(password.to_string()))
    }
}

impl TryFrom<&str> for Password {
    type Error = ParsePasswordError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Password::parse(value)
    }
}

impl TryFrom<String> for Password {
    type Error = ParsePasswordError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        Password::parse(&value)
    }
}

impl AsRef<str> for Password {
    fn as_ref(&self) -> &str {
        self.0.as_str()
    }
}

#[cfg(test)]
mod tests {
    use crate::domain::password::Password;
    use claims::{assert_err, assert_ok};

    #[test]
    fn password_with_at_least_8_chars_special_char_and_number_passes() {
        let password = Password::try_from("password123!");
        assert_ok!(password);
    }

    #[test]
    fn empty_password_is_rejected() {
        let password = Password::try_from("");
        assert_err!(password);
    }

    #[test]
    fn password_missing_special_symbol_is_rejected() {
        let password = Password::try_from("password123");
        assert_err!(password);
    }

    #[test]
    fn password_missing_number_is_rejected() {
        let password = Password::try_from("password!!!");
        assert_err!(password);
    }
}
