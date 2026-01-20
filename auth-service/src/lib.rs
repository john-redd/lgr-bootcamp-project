use crate::services::hashmap_user_store::HashmapUserStore;
use axum::{
    Router,
    routing::{get, post},
    serve::{Serve, serve},
};
use std::{error::Error, sync::Arc};
use tokio::{net::TcpListener, sync::RwLock};
use tower_http::services::ServeDir;

mod domain;
mod routes;
pub mod services;

#[derive(Debug)]
pub struct Application {
    server: Serve<TcpListener, Router, Router>,
    pub address: String,
}

type UserStore = Arc<RwLock<HashmapUserStore>>;

#[derive(Clone)]
pub struct AppState {
    pub user_store: UserStore,
}

impl Application {
    pub async fn build(app_state: AppState, address: &str) -> Result<Self, Box<dyn Error>> {
        let assets_dir = ServeDir::new("assets");
        let app = Router::new()
            .fallback_service(assets_dir)
            .route("/api/v1/health", get(routes::health::get_health))
            .route("/login", post(routes::login::post_login))
            .route("/logout", post(routes::logout::post_logout))
            .route("/signup", post(routes::signup::post_signup))
            .route("/verify-2fa", post(routes::verify_2fa::post_verify_2fa))
            .route(
                "/verify-token",
                post(routes::verify_token::post_verify_token),
            )
            .with_state(app_state);

        let listener = tokio::net::TcpListener::bind(address).await?;
        let address = listener.local_addr()?;

        let server = serve(listener, app);

        Ok(Self {
            server,
            address: address.to_string(),
        })
    }

    pub async fn run(self) -> Result<(), std::io::Error> {
        println!("listening on {}", self.address);
        self.server.await
    }
}
