use axum::{
    routing::{get, post},
    Router,
};
use users::create_user;

mod users;

#[tokio::main]
async fn main() {
    let router = Router::new()
        .route("/", get(|| async { "Hello world" }))
        .route("/users", post(create_user));

    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(router.into_make_service())
        .await
        .unwrap();
}
