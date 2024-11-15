use axum::{
    extract::Json,
    response::IntoResponse,
    http::StatusCode,
};

use std::{
    path::PathBuf,
    fs
};

use crate::models::{Task, CreateTaskRequest};

pub async fn create_task(
    Json(payload): Json<CreateTaskRequest>,
) -> impl IntoResponse {
    let task = Task::new(payload);
    let task_path = PathBuf::from("../tasks/pending")
        .join(format!("{}.json", task.id));

    if let Ok(task_json) = serde_json::to_string(&task) {
        tracing::info!("Task serialized successfully");

        if let Err(e) = fs::write(&task_path, task_json) {
            tracing::error!("Failed to write task file: {}", e);
            return (StatusCode::INTERNAL_SERVER_ERROR, "Failed to create task").into_response();
        }
        else {
            tracing::info!("Task created successfully");
        }
    }
    else {
        tracing::error!("Failed to serialize task");
        return (StatusCode::INTERNAL_SERVER_ERROR, "Failed to serialize task").into_response();
    }

    (StatusCode::CREATED, Json(task)).into_response()
}