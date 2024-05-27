// src/controllers/user_controller.rs

use axum::{extract::Path, http::StatusCode, response::Json};
use serde_json::json;

use crate::models::user::User;
use crate::services::user_service::{check_user, get_user_details};

//user/name
pub async fn get_user(Path(name): Path<String>) -> Json<User> {
    let user = User {
        name: name.clone(),
        age: 21,
        is_alive: true,
    };

    Json(user)
}

//check_user/name
pub async fn check_user_handler(Path(name): Path<String>) -> (StatusCode, Json<serde_json::Value>) {
    let user = User {
        name: name.clone(),
        age: 21,
        is_alive: true,
    };

    let response = json!({
        "details": get_user_details(&user),
        "status": check_user(&user)
    });

    (StatusCode::OK, Json(response))
}
