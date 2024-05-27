// src/main.rs

mod controllers;
mod models;
mod services;

use axum::routing::{get, Router};
use controllers::user_controller::{check_user_handler, get_user};

#[tokio::main]
async fn main() {
    // Build the application with a route
    let app = Router::new()
        .route("/user/:name", get(get_user))
        .route("/check_user/:name", get(check_user_handler));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:8080").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
