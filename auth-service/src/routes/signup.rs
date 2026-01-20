use crate::{AppState, domain::User};
use axum::{Json, extract::State, http::StatusCode, response::IntoResponse};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Debug)]
pub struct SignUpRequestBody {
    #[allow(unused)]
    email: String,
    #[allow(unused)]
    password: String,
    #[allow(unused)]
    #[serde(rename = "requires2FA")]
    requires_2fa: bool,
}

#[derive(Serialize, Debug)]
pub struct SignUpSuccessResponseBody {
    message: String,
}

impl SignUpSuccessResponseBody {
    fn new() -> Self {
        Self {
            message: "User created successfully!".to_string(),
        }
    }
}

#[allow(unused)]
#[derive(Serialize, Debug)]
pub struct SignUpErrorResponseBody {
    error: String,
}

impl SignUpErrorResponseBody {
    #[allow(unused)]
    fn new(error: String) -> Self {
        Self { error }
    }
}

pub async fn post_signup(
    State(app_state): State<AppState>,
    Json(body): Json<SignUpRequestBody>,
) -> impl IntoResponse {
    let user = User::new(body.email, body.password, body.requires_2fa);

    let mut user_store = app_state.user_store.write().await;

    user_store.add_user(user).unwrap();

    (StatusCode::CREATED, Json(SignUpSuccessResponseBody::new()))
}
