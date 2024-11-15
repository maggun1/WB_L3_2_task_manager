mod logger;

use inotify::{Inotify, WatchMask};
use std::path::PathBuf;

async fn main() {
    tracing_subscriber::fmt::init();
    tracing::info!("Task Logger started");

    let task_completed_folder = PathBuf::from("../tasks/completed");
    let task_logger = logger::TaskLogger::new("../tasks/task_log.txt".to_string());

    let mut inotify = Inotify::init()
        .expect("Failed to initialize inotify");

    inotify
        .watches()
        .add(
            task_completed_folder.clone(),
            WatchMask::CREATE | WatchMask::MOVED_TO,
        )
        .expect("Failed to add watch");

    let mut buffer = [0; 4096];
    loop {
        let events = inotify
            .read_events_blocking(&mut buffer)
            .expect("Failed to read events");

        for event in events {
            if let Some(name) = event.name {
                let path = PathBuf::from(task_completed_folder.clone()).join(name);
                task_logger.log_task(&path);
            }
        }
    }
}
