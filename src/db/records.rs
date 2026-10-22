use sqlx::SqlitePool;

use crate::models::record::Record;

pub async fn get_all_records(pool: &SqlitePool) -> Result<Vec<Record>, sqlx::Error> {
    let records = sqlx::query_as::<_, Record>("SELECT id, content FROM records ORDER BY rowid")
        .fetch_all(pool)
        .await?;

    Ok(records)
}
