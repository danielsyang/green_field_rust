use axum::{extract::State, http::StatusCode, response::IntoResponse, Json};
use serde::Deserialize;

use validator::Validate;

use crate::AppState;

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

pub async fn create_user(
    State(s): State<AppState>,
    Json(p): Json<CreateUser>,
) -> impl IntoResponse {
    let payload = p.validate();

    if payload.is_err() {
        return (StatusCode::BAD_REQUEST, payload.err().unwrap().to_string());
    }

    let id = uuid::Uuid::new_v4();

    let result = sqlx::query(
        "INSERT INTO Users (ID, FIRST_NAME, LAST_NAME, EMAIL, PASSWORD) VALUES (? , ? , ? , ? , ?);",
    )
    .bind(id.to_string())
    .bind(p.first_name)
    .bind(p.last_name)
    .bind(p.email)
    .bind(p.password)
    .execute(&s.connection_pool)
    .await;

    match result {
        Ok(_) => (StatusCode::OK, id.to_string()),
        Err(r) => {
            println!("Error: {:#?}", r);

            if r.to_string().contains("2067") {
                return (
                    StatusCode::BAD_REQUEST,
                    String::from("Email already exists."),
                );
            }

            (
                StatusCode::INTERNAL_SERVER_ERROR,
                String::from("Something went wrong."),
            )
        }
    }
}
