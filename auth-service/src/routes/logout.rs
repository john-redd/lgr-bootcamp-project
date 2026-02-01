use crate::{
    constants::JWT_COOKIE_NAME,
    domain::{errors::ErrorResponse, utils::auth::validate_token},
};
use axum::{http::StatusCode, response::IntoResponse};
use axum_extra::extract::{CookieJar, cookie::Cookie};

pub async fn post_logout(jar: CookieJar) -> impl IntoResponse {
    let cookie = match jar.get(JWT_COOKIE_NAME) {
        Some(cookie) => cookie,
        None => {
            return ErrorResponse::new(StatusCode::BAD_REQUEST, "Missing jwt.".to_string())
                .into_response();
        }
    };

    match validate_token(cookie.value_trimmed()).await {
        Ok(_) => {
            let jar = jar.remove(Cookie::from(JWT_COOKIE_NAME));
            (jar, StatusCode::OK).into_response()
        }
        Err(_) => {
            ErrorResponse::new(StatusCode::UNAUTHORIZED, "Invalid jwt.".to_string()).into_response()
        }
    }
}
