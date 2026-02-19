use crate::{
    AppState,
    domain::{errors::ErrorResponse, utils::auth::validate_token},
};
use axum::{Json, extract::State, http::StatusCode, response::IntoResponse};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct VerifyTokenRequestBody {
    token: String,
}

pub async fn post_verify_token(
    State(app_state): State<AppState>,
    Json(body): Json<VerifyTokenRequestBody>,
) -> impl IntoResponse {
    if (app_state.banned_token_store.check_token(&body.token).await).is_some() {
        return ErrorResponse::new(StatusCode::UNAUTHORIZED, "Revoked token.".to_string())
            .into_response();
    };

    match validate_token(&body.token).await {
        Ok(_) => StatusCode::OK.into_response(),
        Err(_) => {
            ErrorResponse::new(StatusCode::UNAUTHORIZED, "Invalid jwt.".to_string()).into_response()
        }
    }
}
