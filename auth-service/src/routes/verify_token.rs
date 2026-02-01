use axum::{Json, http::StatusCode, response::IntoResponse};
use serde::Deserialize;
use crate::domain::{errors::ErrorResponse, utils::auth::validate_token};

#[derive(Debug, Deserialize)]
pub struct VerifyTokenRequestBody {
    token: String,
}

pub async fn post_verify_token(Json(body): Json<VerifyTokenRequestBody>) -> impl IntoResponse {
    match validate_token(&body.token).await {
        Ok(_) => StatusCode::OK.into_response(),
        Err(_) => {
            ErrorResponse::new(StatusCode::UNAUTHORIZED, "Invalid jwt.".to_string()).into_response()
        }
    }
}
