use sqlx::SqlitePool;
use sqlx::sqlite::{SqliteConnectOptions, SqlitePoolOptions};
use std::path::Path;

use crate::db::schema::init_schema;
use crate::terminal::message::print_message;
use crate::terminal::status::Status;

pub async fn init_database(_path: &str) -> Result<SqlitePool, Box<dyn std::error::Error>> {
    let db_path = Path::new(_path);

    if let Some(parent) = db_path.parent() {
        std::fs::create_dir_all(parent)?;
    }

    let existed = db_path.exists();

    let options = SqliteConnectOptions::new()
        .filename(db_path)
        .create_if_missing(true);

    let pool = SqlitePoolOptions::new()
        .max_connections(5)
        .connect_with(options)
        .await?;

    init_schema(&pool).await?;

    if existed {
        print_message("Database initialized successfully", &Status::Success);
    } else {
        print_message("Database created successfully", &Status::Success);
    }

    Ok(pool)
}
