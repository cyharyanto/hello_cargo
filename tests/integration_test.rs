use axum::{
    body::{Body, to_bytes},
    extract::{Path, State},
    http::{Request, StatusCode},
    response::IntoResponse,
    routing::{get, post, put, delete},
    Json, Router,
};
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::sync::Arc;
use tokio::sync::Mutex as AsyncMutex;
use tower::ServiceExt; // for `oneshot` method

#[derive(Serialize, Deserialize, Clone)]
struct User {
    id: usize,
    name: String,
    email: String,
}

struct AppState {
    users: AsyncMutex<Vec<User>>,
}

async fn get_users(State(state): State<Arc<AppState>>) -> impl IntoResponse {
    let users = state.users.lock().await;
    (StatusCode::OK, Json(users.clone()))
}

async fn get_user(State(state): State<Arc<AppState>>, Path(user_id): Path<usize>) -> impl IntoResponse {
    let users = state.users.lock().await;
    if let Some(user) = users.iter().find(|&u| u.id == user_id) {
        (StatusCode::OK, Json(json!(user)))
    } else {
        (StatusCode::NOT_FOUND, Json(json!({"error": "User not found"})))
    }
}

async fn create_user(State(state): State<Arc<AppState>>, Json(new_user): Json<User>) -> impl IntoResponse {
    let mut users = state.users.lock().await;
    users.push(new_user);
    StatusCode::CREATED
}

async fn update_user(State(state): State<Arc<AppState>>, Path(user_id): Path<usize>, Json(updated_user): Json<User>) -> impl IntoResponse {
    let mut users = state.users.lock().await;
    if let Some(user) = users.iter_mut().find(|u| u.id == user_id) {
        *user = updated_user;
        StatusCode::OK
    } else {
        StatusCode::NOT_FOUND
    }
}

async fn delete_user(State(state): State<Arc<AppState>>, Path(user_id): Path<usize>) -> impl IntoResponse {
    let mut users = state.users.lock().await;
    if users.iter().position(|u| u.id == user_id).is_some() {
        users.retain(|u| u.id != user_id);
        StatusCode::OK
    } else {
        StatusCode::NOT_FOUND
    }
}

fn app() -> Router {
    let app_state = Arc::new(AppState {
        users: AsyncMutex::new(vec![]),
    });

    Router::new()
        .route("/users", get(get_users).post(create_user))
        .route("/users/:id", get(get_user).put(update_user).delete(delete_user))
        .with_state(app_state)
}

#[tokio::test]
async fn test_get_users() {
    let app = app();

    let response = app
        .oneshot(Request::builder().uri("/users").body(Body::empty()).unwrap())
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);
}

#[tokio::test]
async fn test_create_user() {
    let app = app();

    let new_user = json!({
        "id": 1,
        "name": "John Doe",
        "email": "john.doe@example.com"
    });

    let response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/users")
                .header("content-type", "application/json")
                .body(Body::from(new_user.to_string()))
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::CREATED);
}

#[tokio::test]
async fn test_get_user() {
    let app = app();

    // First, create a user
    let new_user = json!({
        "id": 1,
        "name": "Hello World",
        "email": "hello.world@example.com"
    });

    app.clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/users")
                .header("content-type", "application/json")
                .body(Body::from(new_user.to_string()))
                .unwrap(),
        )
        .await
        .unwrap();

    // Then, get the user
    let response = app
        .oneshot(Request::builder().uri("/users/1").body(Body::empty()).unwrap())
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);

    let body = to_bytes(response.into_body(), 1024).await.unwrap();
    let user: User = serde_json::from_slice(&body).unwrap();

    assert_eq!(user.name, "Hello World");
    assert_eq!(user.email, "hello.world@example.com");
}