use uuid::Uuid;

pub struct User {
    pub id: Uuid,
    pub email: String,
    pub name: String,
    pub timestamp: chrono::DateTime<chrono::Utc>,
}