use sqlx::SqlitePool;

pub async fn init_schema(pool: &SqlitePool) -> Result<(), sqlx::Error> {
    sqlx::query(
        "CREATE TABLE IF NOT EXISTS records (
            id TEXT PRIMARY KEY,
            content TEXT NOT NULL
        )",
    )
    .execute(pool)
    .await?;

    Ok(())
}
