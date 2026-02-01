use crate::services::UserStore;
use axum::{
    Router,
    http::Method,
    routing::{get, post},
    serve::{Serve, serve},
};
use std::{error::Error, sync::Arc};
use tokio::net::TcpListener;
use tower_http::{cors::CorsLayer, services::ServeDir};

mod domain;
mod routes;
pub mod services;
pub use domain::utils::constants;

#[derive(Debug)]
pub struct Application {
    server: Serve<TcpListener, Router, Router>,
    pub address: String,
}

#[derive(Clone)]
pub struct AppState {
    pub user_store: Arc<dyn UserStore>,
}

impl Application {
    pub async fn build(app_state: AppState, address: &str) -> Result<Self, Box<dyn Error>> {
        let cors_policy = CorsLayer::new()
            .allow_methods([Method::GET, Method::POST])
            .allow_credentials(true)
            .allow_origin(["http://localhost:8000".parse()?]);

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
            .with_state(app_state)
            .layer(cors_policy);

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
