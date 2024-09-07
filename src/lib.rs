use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    routing::{get, post, put, delete},
    Json, Router,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tokio::sync::Mutex as AsyncMutex;
use serde_json::json;

#[derive(Serialize, Deserialize, Clone)]
pub struct User {
    pub id: usize,
    pub name: String,
    pub email: String,
}

pub struct AppState {
    pub users: AsyncMutex<Vec<User>>,
}

pub async fn get_users(State(state): State<Arc<AppState>>) -> impl IntoResponse {
    let users = state.users.lock().await;
    (StatusCode::OK, Json(users.clone()))
}

pub async fn get_user(State(state): State<Arc<AppState>>, Path(user_id): Path<usize>) -> impl IntoResponse {
    let users = state.users.lock().await;
    if let Some(user) = users.iter().find(|&u| u.id == user_id) {
        (StatusCode::OK, Json(json!(user)))
    } else {
        (StatusCode::NOT_FOUND, Json(json!({"error": "User not found"})))
    }
}

pub async fn create_user(State(state): State<Arc<AppState>>, Json(new_user): Json<User>) -> impl IntoResponse {
    let mut users = state.users.lock().await;
    users.push(new_user);
    StatusCode::CREATED
}

pub async fn update_user(State(state): State<Arc<AppState>>, Path(user_id): Path<usize>, Json(updated_user): Json<User>) -> impl IntoResponse {
    let mut users = state.users.lock().await;
    if let Some(user) = users.iter_mut().find(|u| u.id == user_id) {
        *user = updated_user;
        StatusCode::OK
    } else {
        StatusCode::NOT_FOUND
    }
}

pub async fn delete_user(State(state): State<Arc<AppState>>, Path(user_id): Path<usize>) -> impl IntoResponse {
    let mut users = state.users.lock().await;
    if users.iter().position(|u| u.id == user_id).is_some() {
        users.retain(|u| u.id != user_id);
        StatusCode::OK
    } else {
        StatusCode::NOT_FOUND
    }
}

pub fn app() -> Router {
    let app_state = Arc::new(AppState {
        users: AsyncMutex::new(vec![]),
    });

    Router::new()
        .route("/users", get(get_users).post(create_user))
        .route("/users/:id", get(get_user).put(update_user).delete(delete_user))
        .with_state(app_state)
}