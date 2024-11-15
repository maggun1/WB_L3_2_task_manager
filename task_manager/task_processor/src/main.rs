mod watcher;
mod processor;
mod models;

use tokio::sync::mpsc;
use std::path::PathBuf;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();
    tracing::info!("Task Processor started");

    let task_pending_folder = PathBuf::from("../tasks/pending");
    let task_completed_folder = PathBuf::from("../tasks/completed");

    let (sender, mut receiver) = mpsc::channel::<PathBuf>(100);

    let mut file_watcher = watcher::FileWatcher::new(sender, task_pending_folder);
    file_watcher.watch();

    tokio::spawn(async move {
        file_watcher.run().await;
    });

    while let Some(path) = receiver.recv().await {
        tokio::spawn(
            processor::process_task(path, task_completed_folder.clone())
        );
    }
}
