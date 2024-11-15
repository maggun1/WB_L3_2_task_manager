#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub enum TaskStatus {
    Created,
    Completed,
    Failed,
}

#[derive(Debug, serde::Deserialize)]
pub struct CreateTaskRequest {
    pub title: String,
    pub description: String,
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

impl Task {
    pub fn new(request: CreateTaskRequest) -> Self {
        Self {
            id: uuid::Uuid::new_v4().to_string(),
            title: request.title,
            description: request.description,
            status: TaskStatus::Created,
            created_at: chrono::Utc::now().to_string(),
            completed_at: "".to_string(),
        }
    }
}