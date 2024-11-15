mod models;
mod handlers;

use axum::{
    Router,
    routing::post,
};

use std::{
    path::Path,
    fs
};

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();
    tracing::info!("Task Creator started");

    let directories = ["../tasks/pending", "../tasks/completed"];
    for dir in &directories {
        if !Path::new(dir).exists() {
            fs::create_dir_all(dir).expect("Failed to create directory");
        }
    }

    let app = Router::new()
        .route("/task", post(handlers::create_task));

    let addr = "127.0.0.1:3000";
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
    tracing::info!("Task Creator starting on {}", addr);
}