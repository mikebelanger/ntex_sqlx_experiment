#[derive(sqlx::FromRow)]
pub struct Todo {
    pub title: String,
    pub content: String,
}
