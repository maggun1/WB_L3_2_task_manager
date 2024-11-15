#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub enum TaskStatus {
    Created,
    Completed,
    Failed,
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct Task {
    pub id: String,
    pub title: String,
    pub description: String,
    pub status: TaskStatus,
    pub created_at: String,
    pub completed_at: String,
}
