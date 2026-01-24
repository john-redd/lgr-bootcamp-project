use crate::{
    domain::email::Email,
    domain::password::Password,
    domain::user::User,
    services::{UserStore, UserStoreError},
};
use std::collections::HashMap;

#[derive(Default)]
pub struct HashmapUserStore {
    pub users: HashMap<Email, User>,
}

impl HashmapUserStore {
    pub fn new() -> Self {
        Self {
            users: HashMap::new(),
        }
    }
}

#[async_trait::async_trait]
impl UserStore for HashmapUserStore {
    async fn add_user(&mut self, user: User) -> Result<(), UserStoreError> {
        if self.users.contains_key(&user.email) {
            return Err(UserStoreError::UserAlreadyExists);
        }

        self.users.insert(user.email.clone(), user);

        Ok(())
    }

    async fn get_user(&self, email: &Email) -> Result<User, UserStoreError> {
        match self.users.get(email) {
            Some(user) => Ok(user.clone()),
            None => Err(UserStoreError::UserNotFound),
        }
    }

    async fn validate_user(
        &self,
        email: &Email,
        password: &Password,
    ) -> Result<(), UserStoreError> {
        let user = match self.users.get(email) {
            Some(user) => user,
            None => return Err(UserStoreError::UserNotFound),
        };

        if user.password != *password {
            return Err(UserStoreError::InvalidCredentials);
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_add_user() {
        let mut store = HashmapUserStore::new();
        let user = User::parse("john.doe@example.com", "password123!", false).unwrap();

        let result = store.add_user(user).await;

        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_add_user_fails_if_same_email_is_added_twice() {
        let mut store = HashmapUserStore::new();
        let user = User::parse("john.doe@example.com", "password123!", false).unwrap();
        let second_user = User::parse("john.doe@example.com", "password123!", false).unwrap();

        let _ = store.add_user(user).await;
        let result = store.add_user(second_user).await;

        assert!(result.is_err());
        assert!(matches!(
            result.unwrap_err(),
            UserStoreError::UserAlreadyExists
        ));
    }

    #[tokio::test]
    async fn test_get_user() {
        let mut store = HashmapUserStore::new();
        let user = User::parse("john.doe@example.com", "password123!", false).unwrap();

        let _ = store.add_user(user).await;
        let result = store
            .get_user(&Email::try_from("john.doe@example.com").unwrap())
            .await;

        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_get_user_fails_if_user_does_not_exist() {
        let store = HashmapUserStore::new();
        let result = store
            .get_user(&Email::try_from("john.doe@example.com").unwrap())
            .await;

        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), UserStoreError::UserNotFound));
    }

    #[tokio::test]
    async fn test_validate_user() {
        let mut store = HashmapUserStore::new();
        let user = User::parse("john.doe@example.com", "password123!", false).unwrap();

        let email = Email::try_from("john.doe@example.com").unwrap();
        let password = Password::try_from("password123!").unwrap();
        let _ = store.add_user(user).await;
        let result = store.validate_user(&email, &password).await;

        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_validate_user_fails_if_user_does_not_exist() {
        let store = HashmapUserStore::new();

        let email = Email::try_from("john.doe@example.com").unwrap();
        let password = Password::try_from("password123!").unwrap();
        let result = store.validate_user(&email, &password).await;

        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), UserStoreError::UserNotFound));
    }

    #[tokio::test]
    async fn test_validate_user_fails_if_password_is_incorrect() {
        let mut store = HashmapUserStore::new();
        let user = User::parse("john.doe@example.com", "password123!", false).unwrap();

        let _ = store.add_user(user).await;

        let email = Email::try_from("john.doe@example.com").unwrap();
        let password = Password::try_from("password1234!").unwrap();
        let result = store.validate_user(&email, &password).await;

        assert!(result.is_err());
        assert!(matches!(
            result.unwrap_err(),
            UserStoreError::InvalidCredentials
        ));
    }
}
