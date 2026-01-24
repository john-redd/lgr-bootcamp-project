use crate::domain::{
    email::{Email, ParseEmailError},
    password::{ParsePasswordError, Password},
};
use std::fmt::Display;

#[derive(Debug, Clone)]
pub struct User {
    pub email: Email,
    pub password: Password,
    pub requires_2fa: bool,
}

#[derive(Debug)]
pub enum ParseUserError {
    InvalidEmail(String),
    InvalidPassword(String),
}

impl From<ParseEmailError> for ParseUserError {
    fn from(value: ParseEmailError) -> Self {
        Self::InvalidEmail(value.to_string())
    }
}

impl From<ParsePasswordError> for ParseUserError {
    fn from(value: ParsePasswordError) -> Self {
        Self::InvalidPassword(value.to_string())
    }
}

impl Display for ParseUserError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let msg = match self {
            ParseUserError::InvalidEmail(msg) => msg,
            ParseUserError::InvalidPassword(msg) => msg,
        };

        write!(f, "{}", msg)
    }
}

impl User {
    fn new(email: Email, password: Password, requires_2fa: bool) -> Self {
        Self {
            email,
            password,
            requires_2fa,
        }
    }

    pub fn parse(email: &str, password: &str, requires_2fa: bool) -> Result<Self, ParseUserError> {
        let email = Email::try_from(email)?;
        let password = Password::parse(password)?;

        Ok(User::new(email, password, requires_2fa))
    }
}
