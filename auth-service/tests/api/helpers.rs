#![allow(clippy::let_underscore_future)]

use authservice::{AppState, Application, services::hashmap_user_store::HashmapUserStore};
use std::{error::Error, sync::Arc};
use tokio::sync::RwLock;

pub struct TestApp {
    address: String,
    http_client: reqwest::Client,
}

impl TestApp {
    pub async fn build() -> Result<Self, Box<dyn Error>> {
        let user_store = HashmapUserStore::new();
        let app_state = AppState {
            user_store: Arc::new(RwLock::new(user_store)),
        };
        let application = Application::build(app_state, "127.0.0.1:0")
            .await
            .expect("failed to build test app");

        let address = application.address.clone();

        let _ = tokio::spawn(application.run());

        let test_app = TestApp {
            address,
            http_client: reqwest::Client::new(),
        };

        Ok(test_app)
    }

    pub fn base_url(&self) -> String {
        format!("http://{}", self.address)
    }

    pub async fn get_heatlh(&self) -> reqwest::Response {
        let request_url = format!("{}/api/v1/health", self.base_url());
        self.http_client
            .get(request_url)
            .send()
            .await
            .expect("failed to GET /api/v1/health")
    }

    pub async fn get_root(&self) -> reqwest::Response {
        let request_url = self.base_url();
        self.http_client
            .get(request_url)
            .send()
            .await
            .expect("failed to GET /")
    }

    pub async fn post_signup(&self, body: Option<String>) -> reqwest::Response {
        let request_url = format!("{}/signup", self.base_url());

        let mut request = self
            .http_client
            .post(request_url)
            .header("Content-Type", "application/json");

        if let Some(body) = body {
            request = request.body(body);
        }

        request.send().await.expect("failed to POST /signup")
    }

    pub async fn post_login(&self) -> reqwest::Response {
        let request_url = format!("{}/login", self.base_url());
        self.http_client
            .post(request_url)
            .send()
            .await
            .expect("failed to POST /login")
    }

    pub async fn post_verify_2fa(&self) -> reqwest::Response {
        let request_url = format!("{}/verify-2fa", self.base_url());
        self.http_client
            .post(request_url)
            .send()
            .await
            .expect("failed to POST /verify-2fa")
    }

    pub async fn post_logout(&self) -> reqwest::Response {
        let request_url = format!("{}/logout", self.base_url());
        self.http_client
            .post(request_url)
            .send()
            .await
            .expect("failed to POST /logout")
    }

    pub async fn post_verify_token(&self) -> reqwest::Response {
        let request_url = format!("{}/verify-token", self.base_url());
        self.http_client
            .post(request_url)
            .send()
            .await
            .expect("failed to POST /verify-token")
    }
}
