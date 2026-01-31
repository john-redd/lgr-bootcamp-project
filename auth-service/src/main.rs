use authservice::{AppState, Application, services::hashmap_user_store::HashmapUserStore};
use std::sync::Arc;

#[tokio::main]
async fn main() {
    let user_store = HashmapUserStore::new();
    let app_state = AppState {
        user_store: Arc::new(user_store),
    };
    let application = Application::build(app_state, "0.0.0.0:3000")
        .await
        .expect("failed to build app");

    application.run().await.expect("failed to run app")
}
