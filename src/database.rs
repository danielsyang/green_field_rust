use sqlx::{migrate::MigrateDatabase, sqlite::SqlitePoolOptions, Pool, Sqlite};

// make it env variable
const DB_URL: &str = "sqlite://sqlite.db";

pub async fn connection() -> Pool<Sqlite> {
    if !Sqlite::database_exists(DB_URL).await.unwrap_or(false) {
        println!("Creating database...");

        match Sqlite::create_database(DB_URL).await {
            Ok(_) => println!("DB created successfully"),
            Err(err) => panic!("Error: {:#?}", err),
        }
    };

    let pool = SqlitePoolOptions::new()
        .max_connections(5)
        .connect(DB_URL)
        .await
        .unwrap();

    sqlx::query(
        "
        CREATE TABLE if not EXISTS Users (
            ID TEXT PRIMARY KEY,
            FIRST_NAME TEXT NOT NULL,
            LAST_NAME TEXT NOT NULL,
            EMAIL TEXT NOT NULL UNIQUE,
            PASSWORD TEXT NOT NULL,
            UPDATED_AT DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
            CREATED_AT DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP
        )",
    )
    .execute(&pool)
    .await
    .unwrap();

    pool
}
