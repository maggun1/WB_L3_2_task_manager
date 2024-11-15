use std::{
    path::PathBuf,
    fs
};

use crate::models::{
    Task,
    TaskStatus
};

pub async fn process_task(pending_task_path: PathBuf, completed_task_path: PathBuf) {
    let content = match fs::read_to_string(&pending_task_path) {
        Ok(content) => {
            tracing::info!("Processing task: {}", pending_task_path.display());
            content
        },
        Err(e) => {
            tracing::error!("Failed to read task file: {}", e);
            return;
        }
    };

    let mut task: Task = match serde_json::from_str(&content) {
        Ok(task) => {
            tracing::info!("Task serialized: {}", pending_task_path.display());
            task
        },
        Err(e) => {
            tracing::error!("Failed to serialize task: {}", e);
            return;
        }
    };


    // Имитация обработки задачи. В данном случае мы просто задерживаем выполнение на 10 секунд
    tokio::time::sleep(tokio::time::Duration::from_secs(10)).await;
    task.status = TaskStatus::Completed;
    task.completed_at = chrono::Utc::now().to_string();


    let completed_task_path = completed_task_path.join(pending_task_path.file_name().unwrap());
    if let Ok(task_json) = serde_json::to_string(&task) {
        if let Err(e) = fs::write(&completed_task_path, task_json) {
            tracing::error!("Failed to write completed task: {}", e);
            return;
        }
        else {
            tracing::info!("Completed task written: {}", completed_task_path.display());
        }
    }
    else {
        tracing::error!("Failed to serialize completed task");
    }

    if let Err(e) = fs::remove_file(&pending_task_path) {
        tracing::error!("Failed to remove original task file: {}", e);
    }
    else {
        tracing::info!("Original task file removed: {}", pending_task_path.display());
    }
}
