use crate::domain::email::Email;
use crate::domain::password::Password;
use crate::domain::user::User;

pub mod hashmap_user_store;

#[derive(Debug)]
pub enum UserStoreError {
    UserAlreadyExists,
    UserNotFound,
    InvalidCredentials,
    UnexpectedError,
}

#[async_trait::async_trait]
pub trait UserStore: Send + Sync {
    async fn add_user(&mut self, user: User) -> Result<(), UserStoreError>;

    async fn get_user(&self, email: &Email) -> Result<User, UserStoreError>;

    async fn validate_user(&self, email: &Email, password: &Password)
    -> Result<(), UserStoreError>;
}
