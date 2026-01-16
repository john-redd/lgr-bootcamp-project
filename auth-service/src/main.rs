use axum::{Router, http::StatusCode, response::IntoResponse, routing::get, serve};
use tower_http::services::ServeDir;

#[tokio::main]
async fn main() {
    let assets_dir = ServeDir::new("assets");
    let app = Router::new()
        .fallback_service(assets_dir)
        .route("/api/v1/health", get(health_handler));

    // Here we are using ip 0.0.0.0 so the service is listening on all the configured network interfaces.
    // This is needed for Docker to work, which we will add later on.
    // See: https://stackoverflow.com/questions/39525820/docker-port-forwarding-not-working
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    println!("listening on {}", listener.local_addr().unwrap());

    serve(listener, app).await.unwrap();
}

async fn health_handler() -> impl IntoResponse {
    (StatusCode::OK, "OK")
}
