use crate::services::{BannedTokenStore, TokenStoreError};
use std::collections::HashSet;
use tokio::sync::RwLock;

pub struct HashSetBannedTokenStore {
    store: RwLock<HashSet<String>>,
}

impl HashSetBannedTokenStore {
    pub fn new() -> Self {
        Self {
            store: RwLock::new(HashSet::new()),
        }
    }
}

#[async_trait::async_trait]
impl BannedTokenStore for HashSetBannedTokenStore {
    async fn add_token(&self, token: &str) -> Result<(), TokenStoreError> {
        let mut guard = self.store.write().await;

        guard.insert(token.to_string());

        Ok(())
    }

    async fn check_token(&self, token: &str) -> Option<()> {
        if self.store.read().await.contains(token) {
            Some(())
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_add_token() {
        let store = HashSetBannedTokenStore::new();

        let token = "fake-token";

        let result = store.add_token(token).await;

        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_add_token_is_idempotent() {
        let store = HashSetBannedTokenStore::new();

        let token = "fake-token";

        let _ = store.add_token(token).await;
        let result = store.add_token(token).await;

        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_check_token_succeeds_if_token_exists() {
        let store = HashSetBannedTokenStore::new();

        let token = "fake-token";

        let _ = store.add_token(token).await;

        let result = store.check_token(token).await;

        assert!(result.is_some());
    }

    #[tokio::test]
    async fn test_check_token_returns_none_if_token_does_not_exist() {
        let store = HashSetBannedTokenStore::new();

        let token = "fake-token";

        let result = store.check_token(token).await;

        assert!(result.is_none());
    }
}
