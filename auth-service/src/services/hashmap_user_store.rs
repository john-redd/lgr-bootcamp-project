use crate::domain::User;
use std::collections::HashMap;

#[derive(Debug)]
pub enum UserStoreError {
    UserAlreadyExists,
    UserNotFound,
    InvalidCredentials,
    UnexpectedError,
}

#[derive(Default)]
pub struct HashmapUserStore {
    pub users: HashMap<String, User>,
}

impl HashmapUserStore {
    pub fn new() -> Self {
        Self {
            users: HashMap::new(),
        }
    }

    pub fn add_user(&mut self, user: User) -> Result<(), UserStoreError> {
        if self.users.contains_key(&user.email) {
            return Err(UserStoreError::UserAlreadyExists);
        }

        self.users.insert(user.email.clone(), user);

        Ok(())
    }

    pub fn get_user(&self, email: &str) -> Result<User, UserStoreError> {
        match self.users.get(email) {
            Some(user) => Ok(user.clone()),
            None => Err(UserStoreError::UserNotFound),
        }
    }

    pub fn validate_user(&self, email: &str, password: &str) -> Result<(), UserStoreError> {
        let user = match self.users.get(email) {
            Some(user) => user,
            None => return Err(UserStoreError::UserNotFound),
        };

        if user.password != password {
            return Err(UserStoreError::InvalidCredentials);
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_user() {
        let mut store = HashmapUserStore::new();
        let user = User::new(
            "john.doe@example.com".to_string(),
            "password123".to_string(),
            false,
        );

        let result = store.add_user(user);

        assert!(result.is_ok());
    }

    #[test]
    fn test_add_user_fails_if_same_email_is_added_twice() {
        let mut store = HashmapUserStore::new();
        let user = User::new(
            "john.doe@example.com".to_string(),
            "password123".to_string(),
            false,
        );
        let second_user = User::new(
            "john.doe@example.com".to_string(),
            "password123".to_string(),
            false,
        );

        let _ = store.add_user(user);
        let result = store.add_user(second_user);

        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), UserStoreError::UserAlreadyExists));
    }

    #[test]
    fn test_get_user() {
        let mut store = HashmapUserStore::new();
        let user = User::new(
            "john.doe@example.com".to_string(),
            "password123".to_string(),
            false,
        );

        let _ = store.add_user(user);
        let result = store.get_user("john.doe@example.com");

        assert!(result.is_ok());
    }

    #[test]
    fn test_get_user_fails_if_user_does_not_exist() {
        let store = HashmapUserStore::new();
        let result = store.get_user("john.doe@example.com");

        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), UserStoreError::UserNotFound));
    }

    #[test]
    fn test_validate_user() {
        let mut store = HashmapUserStore::new();
        let user = User::new(
            "john.doe@example.com".to_string(),
            "password123".to_string(),
            false,
        );

        let _ = store.add_user(user);
        let result = store.validate_user("john.doe@example.com", "password123");

        assert!(result.is_ok());
    }

    #[test]
    fn test_validate_user_fails_if_user_does_not_exist() {
        let store = HashmapUserStore::new();

        let result = store.validate_user("john.doe@example.com", "password123");

        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), UserStoreError::UserNotFound));
    }

    #[test]
    fn test_validate_user_fails_if_password_is_incorrect() {
        let mut store = HashmapUserStore::new();
        let user = User::new(
            "john.doe@example.com".to_string(),
            "password123".to_string(),
            false,
        );

        let _ = store.add_user(user);
        let result = store.validate_user("john.doe@example.com", "password");

        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), UserStoreError::InvalidCredentials));
    }
}
