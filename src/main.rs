use axum::{
    http::StatusCode,
    routing::{get, post},
    Router,
};
use database::connection;
use sqlx::{Pool, Sqlite};
use users::create_user;

mod database;
mod users;

#[derive(Clone)]
pub struct AppState {
    connection_pool: Pool<Sqlite>,
}

#[tokio::main]
async fn main() {
    let pool = connection().await;

    let state = AppState {
        connection_pool: pool,
    };

    let router = Router::new()
        .route("/health-check", get(|| async { (StatusCode::OK, "OK") }))
        .route("/users", post(create_user))
        .with_state(state);

    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(router.into_make_service())
        .await
        .unwrap();
}
