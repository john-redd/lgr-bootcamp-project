use std::fmt::Display;

pub mod errors;

#[derive(Debug, Clone)]
pub struct User {
    pub email: String,
    pub password: String,
    pub requires_2fa: bool,
}

#[derive(Debug)]
pub enum ParseUserError {
    InvalidEmail(String),
    InvalidPassword(String),
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
    fn new(email: String, password: String, requires_2fa: bool) -> Self {
        Self {
            email,
            password,
            requires_2fa,
        }
    }

    pub fn parse(
        email: String,
        password: String,
        requires_2fa: bool,
    ) -> Result<Self, ParseUserError> {
        if !email.contains('@') {
            return Err(ParseUserError::InvalidEmail(format!(
                "{email} is missing an @ symbol"
            )));
        }

        if password.len() < 8 {
            return Err(ParseUserError::InvalidEmail(
                "password is too short, it must contain at least 8 characters".to_string(),
            ));
        }

        Ok(User::new(email, password, requires_2fa))
    }
}
