use std::fs::{self, OpenOptions};
use std::io::Write;
use std::path::Path;

pub struct TaskLogger {
    log_file: String,
}

impl TaskLogger {
    pub fn new(log_file: String) -> Self {
        Self { log_file }
    }

    pub fn log_task(&self, task_path: &Path) {
        if let Ok(content) = fs::read_to_string(task_path) {
            if let Ok(mut file) = OpenOptions::new()
                .create(true)
                .append(true)
                .open(&self.log_file)
            {
                let mut log: String = String::new();
                log.push_str("=".repeat(300).as_str());
                log.push('\n');
                log.push_str("Completed Task\n");
                log.push_str(format!("Time: {}\n", chrono::Utc::now()).as_str());
                log.push_str(format!("Path: {}\n", task_path.display()).as_str());
                log.push_str(format!("Content: {}\n", content).as_str());
                log.push_str("=".repeat(300).as_str());
                log.push('\n');

                if let Err(e) = writeln!(file, "{}", log) {
                    tracing::error!("Failed to write to log: {}", e);
                }
                else {
                    tracing::info!("Task logged: {}", task_path.display());
                }
            }
            else {
                tracing::error!("Failed to open log file");
            }
        }
        else {
            tracing::error!("Failed to read task file");
        }
    }
}