use crate::{
    AppState,
    domain::{email::Email, errors::ErrorResponse, password::Password},
    services::UserStoreError,
};
use axum::{Json, extract::State, http::StatusCode, response::IntoResponse};
use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct LoginRequestBody {
    email: String,
    password: String,
}

pub async fn post_login(
    State(app_state): State<AppState>,
    Json(body): Json<LoginRequestBody>,
) -> impl IntoResponse {
    let email = match Email::parse(&body.email) {
        Ok(email) => email,
        Err(e) => {
            return ErrorResponse::new(StatusCode::BAD_REQUEST, e.to_string()).into_response();
        }
    };
    let password = match Password::parse(&body.password) {
        Ok(password) => password,
        Err(_e) => {
            return ErrorResponse::new(StatusCode::BAD_REQUEST, "Invalid credentials.".to_string())
                .into_response();
        }
    };

    let is_valid_password = app_state.user_store.validate_user(&email, &password).await;

    if let Err(e) = is_valid_password {
        match e {
            UserStoreError::UserNotFound | UserStoreError::InvalidCredentials => {
                return ErrorResponse::new(
                    StatusCode::UNAUTHORIZED,
                    "Invalid credentials.".to_string(),
                )
                .into_response();
            }
            _ => {
                return ErrorResponse::new(
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "Unexpected error, please try again later.".to_string(),
                )
                .into_response();
            }
        }
    }

    StatusCode::OK.into_response()
}
