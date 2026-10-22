#[derive(Debug, Clone, sqlx::FromRow)]
pub struct Record {
    pub id: String,
    pub content: String,
}
