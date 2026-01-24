use crate::{
    AppState, domain::errors::ErrorResponse, domain::user::User, services::UserStoreError,
};
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

pub async fn post_signup(
    State(app_state): State<AppState>,
    Json(body): Json<SignUpRequestBody>,
) -> Result<impl IntoResponse, impl IntoResponse> {
    let user = match User::parse(&body.email, &body.password, body.requires_2fa) {
        Ok(user) => user,
        Err(e) => {
            return Err(ErrorResponse::new(
                StatusCode::UNPROCESSABLE_ENTITY,
                e.to_string(),
            ));
        }
    };

    let mut user_store = app_state.user_store.write().await;

    let add_user_result = user_store.add_user(user).await;

    if add_user_result.is_err() {
        match add_user_result.err() {
            Some(UserStoreError::UserAlreadyExists) => {
                return Err(ErrorResponse::new(
                    StatusCode::CONFLICT,
                    "Email already exists.".to_string(),
                ));
            }
            None | Some(_) => {
                return Err(ErrorResponse::new(
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "Unexpected error".to_string(),
                ));
            }
        }
    }

    Ok((StatusCode::CREATED, Json(SignUpSuccessResponseBody::new())))
}
