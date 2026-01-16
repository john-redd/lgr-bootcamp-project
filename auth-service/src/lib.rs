use axum::{
    Router,
    http::StatusCode,
    response::IntoResponse,
    routing::get,
    serve::{Serve, serve},
};
use std::error::Error;
use tokio::net::TcpListener;
use tower_http::services::ServeDir;

#[derive(Debug)]
pub struct Application {
    server: Serve<TcpListener, Router, Router>,
    pub address: String,
}

impl Application {
    pub async fn build(address: &str) -> Result<Self, Box<dyn Error>> {
        let assets_dir = ServeDir::new("assets");
        let app = Router::new()
            .fallback_service(assets_dir)
            .route("/api/v1/health", get(health_handler));

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

async fn health_handler() -> impl IntoResponse {
    (StatusCode::OK, "OK")
}
