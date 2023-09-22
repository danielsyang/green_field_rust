use axum::{http::StatusCode, response::IntoResponse, Json};
use serde::Deserialize;

use validator::Validate;

#[derive(Debug, Deserialize, Validate)]
pub struct CreateUser {
    #[serde(rename = "firstName")]
    first_name: String,
    #[serde(rename = "lastName")]
    last_name: String,
    #[validate(length(min = 6, message = "Password must be at least 6 characters"))]
    password: String,
    #[validate(email(message = "Invalid email"))]
    email: String,
}

pub async fn create_user(Json(p): Json<CreateUser>) -> impl IntoResponse {
    let payload = p.validate();

    if payload.is_err() {
        return (StatusCode::BAD_REQUEST, payload.err().unwrap().to_string());
    }

    return (StatusCode::OK, String::from("User created"));
}
