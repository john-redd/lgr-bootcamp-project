#![allow(clippy::let_underscore_future)]

use authservice::{
    AppState, Application, constants::test, services::hashmap_user_store::HashmapUserStore,
};
use reqwest::{cookie::Jar, header::CONTENT_TYPE};
use serde::Serialize;
use std::{error::Error, sync::Arc};

pub struct TestApp {
    address: String,
    http_client: reqwest::Client,
    pub cookie_jar: Arc<Jar>,
}

impl TestApp {
    pub async fn build() -> Result<Self, Box<dyn Error>> {
        let user_store = HashmapUserStore::new();
        let app_state = AppState {
            user_store: Arc::new(user_store),
        };
        let application = Application::build(app_state, test::APP_ADDRESS)
            .await
            .expect("failed to build test app");

        let address = application.address.clone();

        let _ = tokio::spawn(application.run());

        let jar = Arc::new(Jar::default());

        let test_app = TestApp {
            address,
            http_client: reqwest::Client::builder()
                .cookie_provider(jar.clone())
                .build()
                .unwrap(),
            cookie_jar: jar,
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

    pub async fn post_signup<Body>(&self, body: &Body) -> reqwest::Response
    where
        Body: Serialize,
    {
        let request_url = format!("{}/signup", self.base_url());

        self.http_client
            .post(request_url)
            .header("Content-Type", "application/json")
            .json(body)
            .send()
            .await
            .expect("failed to POST /signup")
    }

    pub async fn post_login<Body>(&self, request_body: &Body) -> reqwest::Response
    where
        Body: Serialize,
    {
        let request_url = format!("{}/login", self.base_url());
        self.http_client
            .post(request_url)
            .header(CONTENT_TYPE, "application/json")
            .json(request_body)
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

    pub async fn post_verify_token<Body>(&self, body: &Body) -> reqwest::Response
    where
        Body: Serialize,
    {
        let request_url = format!("{}/verify-token", self.base_url());
        self.http_client
            .post(request_url)
            .header(CONTENT_TYPE, "application/json")
            .json(body)
            .send()
            .await
            .expect("failed to POST /verify-token")
    }
}
