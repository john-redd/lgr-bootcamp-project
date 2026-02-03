use authservice::{
    AppState, Application,
    constants::prod,
    services::{
        hashmap_user_store::HashmapUserStore, hashset_banned_token_store::HashSetBannedTokenStore,
    },
};
use std::sync::Arc;

#[tokio::main]
async fn main() {
    let user_store = HashmapUserStore::new();
    let banned_token_store = HashSetBannedTokenStore::new();
    let app_state = AppState {
        user_store: Arc::new(user_store),
        banned_token_store: Arc::new(banned_token_store),
    };
    let application = Application::build(app_state, prod::APP_ADDRESS)
        .await
        .expect("failed to build app");

    application.run().await.expect("failed to run app")
}
