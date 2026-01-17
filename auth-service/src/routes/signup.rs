use axum::{Json, http::StatusCode, response::IntoResponse};
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

pub async fn post_signup(Json(_body): Json<SignUpRequestBody>) -> impl IntoResponse {
    (StatusCode::CREATED, Json(SignUpSuccessResponseBody::new()))
}
