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
use utoipa::OpenApi;
use utoipa::ToSchema;
use utoipa_swagger_ui::SwaggerUi;

#[derive(Serialize, Deserialize, Clone, ToSchema)]
pub struct User {
    #[schema(example = 1)]
    pub id: usize,
    #[schema(example = "John Doe")]
    pub name: String,
    #[schema(example = "john.doe@example.com")]
    pub email: String,
}

pub struct AppState {
    pub users: AsyncMutex<Vec<User>>,
}

#[utoipa::path(
    get,
    path = "/users",
    responses(
        (status = 200, description = "List of users", body = Vec<User>)
    )
)]
pub async fn get_users(State(state): State<Arc<AppState>>) -> impl IntoResponse {
    let users = state.users.lock().await;
    (StatusCode::OK, Json(users.clone()))
}

#[utoipa::path(
    get,
    path = "/users/{user_id}",
    responses(
        (status = 200, description = "User found", body = User),
        (status = 404, description = "User not found")
    ),
    params(
        ("user_id" = usize, Path, description = "User ID")
    )
)]
pub async fn get_user(State(state): State<Arc<AppState>>, Path(user_id): Path<usize>) -> impl IntoResponse {
    let users = state.users.lock().await;
    if let Some(user) = users.iter().find(|&u| u.id == user_id) {
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
        (status = 201, description = "User created successfully")
    )
)]
pub async fn create_user(State(state): State<Arc<AppState>>, Json(new_user): Json<User>) -> impl IntoResponse {
    let mut users = state.users.lock().await;
    users.push(new_user);
    StatusCode::CREATED
}

#[utoipa::path(
    put,
    path = "/users/{user_id}",
    request_body = User,
    responses(
        (status = 200, description = "User updated successfully"),
        (status = 404, description = "User not found")
    ),
    params(
        ("user_id" = usize, Path, description = "User ID")
    )
)]
pub async fn update_user(State(state): State<Arc<AppState>>, Path(user_id): Path<usize>, Json(updated_user): Json<User>) -> impl IntoResponse {
    let mut users = state.users.lock().await;
    if let Some(user) = users.iter_mut().find(|u| u.id == user_id) {
        *user = updated_user;
        StatusCode::OK
    } else {
        StatusCode::NOT_FOUND
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
        ("user_id" = usize, Path, description = "User ID")
    )
)]
pub async fn delete_user(State(state): State<Arc<AppState>>, Path(user_id): Path<usize>) -> impl IntoResponse {
    let mut users = state.users.lock().await;
    if users.iter().position(|u| u.id == user_id).is_some() {
        users.retain(|u| u.id != user_id);
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

pub fn app() -> Router {
    let app_state = Arc::new(AppState {
        users: AsyncMutex::new(vec![]),
    });

    Router::new()
        .merge(SwaggerUi::new("/swagger-ui").url("/api-docs/openapi.json", ApiDoc::openapi()))
        .route("/users", get(get_users).post(create_user))
        .route("/users/:id", get(get_user).put(update_user).delete(delete_user))
        .with_state(app_state)
}