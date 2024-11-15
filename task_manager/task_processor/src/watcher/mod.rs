use inotify::{
    Inotify,
    WatchMask
};
use std::path::PathBuf;
use tokio::sync::mpsc;

pub struct FileWatcher {
    inotify: Inotify,
    sender: mpsc::Sender<PathBuf>,
    pending_task_path: PathBuf,
}

impl FileWatcher {
    pub fn new(sender: mpsc::Sender<PathBuf>, pending_task_path: PathBuf) -> Self {
        Self {
            inotify: Inotify::init().expect("Failed to initialize inotify"),
            sender: sender,
            pending_task_path: pending_task_path,
        }
    }

    pub fn watch(&mut self) {
        self.inotify
            .watches()
            .add(self.pending_task_path.clone(),
                 WatchMask::CREATE)
            .expect("Failed to add watch");
    }

    pub async fn run(self) {
        let mut buffer = [0; 4096];
        let mut inotify = self.inotify;

        loop {
            let events = inotify
                .read_events_blocking(&mut buffer)
                .expect("Failed to read events");

            for event in events {
                if let Some(name) = event.name {
                    let path = self.pending_task_path.join(name);
                    if let Err(e) = self.sender.send(path.clone()).await {
                        tracing::error!("Failed to send path: {}", e);
                    }
                    else {
                        tracing::info!("Sent path: {}", path.display());
                    }
                }
                else {
                    tracing::error!("Failed to get name from event: {:?}", event);
                }
            }
        }
    }
}