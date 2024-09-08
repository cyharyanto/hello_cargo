mod services;
pub mod repositories;
mod models;
mod schema;

use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    routing::get,
    Json, Router,
};
use std::sync::Arc;
use serde_json::json;
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

use services::UserService;
use crate::repositories::UserRepositoryArc;

// Re-export User for use in tests
pub use models::User;

pub struct AppState {
    user_service: Arc<UserService>,
}

#[utoipa::path(
    get,
    path = "/users",
    responses(
        (status = 200, description = "List of users", body = Vec<User>)
    )
)]
async fn get_users(State(state): State<Arc<AppState>>) -> impl IntoResponse {
    let users = state.user_service.get_all_users().await;
    (StatusCode::OK, Json(users))
}

#[utoipa::path(
    get,
    path = "/users/{user_id}",
    responses(
        (status = 200, description = "User found", body = User),
        (status = 404, description = "User not found")
    ),
    params(
        ("user_id" = String, Path, description = "User ID")
    )
)]
async fn get_user(State(state): State<Arc<AppState>>, Path(user_id): Path<String>) -> impl IntoResponse {
    if let Some(user) = state.user_service.get_user(&user_id).await {
        (StatusCode::OK, Json(json!(user)))
    } else {
        (StatusCode::NOT_FOUND, Json(json!({"error": "User not found"})))
    }
}

#[utoipa::path(
    post,
    path = "/users",
    request_body = User,
    responses(
        (status = 201, description = "User created successfully", body = User),
        (status = 400, description = "User ID already exists")
    )
)]
async fn create_user(State(state): State<Arc<AppState>>, Json(new_user): Json<User>) -> impl IntoResponse {
    match state.user_service.create_user(new_user).await {
        Ok(created_user) => (StatusCode::CREATED, Json(json!(created_user))),
        Err(e) => (StatusCode::BAD_REQUEST, Json(json!({"error": e}))),
    }
}

#[utoipa::path(
    put,
    path = "/users/{user_id}",
    request_body = User,
    responses(
        (status = 200, description = "User updated successfully"),
        (status = 400, description = "New user ID already exists"),
        (status = 404, description = "User not found")
    ),
    params(
        ("user_id" = String, Path, description = "User ID")
    )
)]
async fn update_user(
    State(state): State<Arc<AppState>>,
    Path(user_id): Path<String>,
    Json(updated_user): Json<User>
) -> impl IntoResponse {
    match state.user_service.update_user(&user_id, updated_user).await {
        Ok(_) => (StatusCode::OK, Json(json!({"message": "User updated successfully"}))),
        Err(e) => {
            let status = if e == "User not found" { StatusCode::NOT_FOUND } else { StatusCode::BAD_REQUEST };
            (status, Json(json!({"error": e})))
        }
    }
}

#[utoipa::path(
    delete,
    path = "/users/{user_id}",
    responses(
        (status = 200, description = "User deleted successfully"),
        (status = 404, description = "User not found")
    ),
    params(
        ("user_id" = String, Path, description = "User ID")
    )
)]
async fn delete_user(State(state): State<Arc<AppState>>, Path(user_id): Path<String>) -> impl IntoResponse {
    if state.user_service.delete_user(&user_id).await {
        StatusCode::OK
    } else {
        StatusCode::NOT_FOUND
    }
}

#[derive(OpenApi)]
#[openapi(
    paths(
        get_users,
        get_user,
        create_user,
        update_user,
        delete_user
    ),
    components(
        schemas(User)
    ),
    tags(
        (name = "users", description = "User management API")
    )
)]
struct ApiDoc;

pub fn app(user_repository: UserRepositoryArc) -> Router {
    let user_service = Arc::new(UserService::new(user_repository));
    let app_state = Arc::new(AppState { user_service });

    Router::new()
        .merge(SwaggerUi::new("/swagger-ui").url("/api-docs/openapi.json", ApiDoc::openapi()))
        .route("/users", get(get_users).post(create_user))
        .route("/users/:id", get(get_user).put(update_user).delete(delete_user))
        .with_state(app_state)
}