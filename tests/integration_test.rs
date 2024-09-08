use std::sync::Arc;
use axum::{
    body::{Body, to_bytes},
    http::{Request, StatusCode},
};
use hello_cargo::{app, User};
use serde_json::json;
use tower::ServiceExt;
use ulid::Ulid;

use hello_cargo::repositories::in_memory_repository::InMemoryUserRepository;

#[tokio::test]
async fn test_get_users() {
    let user_repository = Arc::new(InMemoryUserRepository::new()) as Arc<dyn hello_cargo::repositories::UserRepository>;
    let app = app(user_repository);

    let response = app
        .oneshot(Request::builder().uri("/users").body(Body::empty()).unwrap())
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);
}

#[tokio::test]
async fn test_create_user() {
    let user_repository = Arc::new(InMemoryUserRepository::new()) as Arc<dyn hello_cargo::repositories::UserRepository>;
    let app = app(user_repository);

    let new_user = json!({
        "id": "",  // Empty string for auto-generated ULID
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

    let body = to_bytes(response.into_body(), 1024).await.unwrap();
    let created_user: serde_json::Value = serde_json::from_slice(&body).unwrap();

    assert!(!created_user["id"].as_str().unwrap().is_empty());
    assert_eq!(created_user["name"], "John Doe");
    assert_eq!(created_user["email"], "john.doe@example.com");
}

#[tokio::test]
async fn test_get_user() {
    let user_repository = Arc::new(InMemoryUserRepository::new()) as Arc<dyn hello_cargo::repositories::UserRepository>;
    let app = app(user_repository);

    // First, create a user
    let new_user = json!({
        "id": "",  // Empty string for auto-generated ULID
        "name": "Hello World",
        "email": "hello.world@example.com"
    });

    let create_response = app
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

    assert_eq!(create_response.status(), StatusCode::CREATED);

    // Now, get all users to find the ID of the created user
    let get_all_response = app
        .clone()
        .oneshot(Request::builder().uri("/users").body(Body::empty()).unwrap())
        .await
        .unwrap();

    assert_eq!(get_all_response.status(), StatusCode::OK);

    let body = to_bytes(get_all_response.into_body(), 1024).await.unwrap();
    let users: Vec<User> = serde_json::from_slice(&body).unwrap();

    assert_eq!(users.len(), 1);
    let user_id = &users[0].id;

    // Then, get the specific user
    let response = app
        .oneshot(Request::builder().uri(&format!("/users/{}", user_id)).body(Body::empty()).unwrap())
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);

    let body = to_bytes(response.into_body(), 1024).await.unwrap();
    let user: User = serde_json::from_slice(&body).unwrap();

    assert_eq!(user.name, "Hello World");
    assert_eq!(user.email, "hello.world@example.com");
}

#[tokio::test]
async fn test_create_duplicate_user() {
    let user_repository = Arc::new(InMemoryUserRepository::new()) as Arc<dyn hello_cargo::repositories::UserRepository>;
    let app = app(user_repository);

    let user_id = Ulid::new().to_string();
    let new_user = json!({
        "id": user_id,
        "name": "John Doe",
        "email": "john.doe@example.com"
    });

    // Create the first user
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

    // Attempt to create a duplicate user
    let response = app
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

    assert_eq!(response.status(), StatusCode::BAD_REQUEST);

    let body = to_bytes(response.into_body(), 1024).await.unwrap();
    let error: serde_json::Value = serde_json::from_slice(&body).unwrap();

    assert_eq!(error, json!({"error": "User ID already exists"}));
}

#[tokio::test]
async fn test_update_user_id_conflict() {
    let user_repository = Arc::new(InMemoryUserRepository::new()) as Arc<dyn hello_cargo::repositories::UserRepository>;
    let app = app(user_repository);

    // Create user 1
    let user1_id = Ulid::new().to_string();
    let user1 = json!({
        "id": user1_id,
        "name": "User One",
        "email": "user.one@example.com"
    });
    app.clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/users")
                .header("content-type", "application/json")
                .body(Body::from(user1.to_string()))
                .unwrap(),
        )
        .await
        .unwrap();

    // Create user 2
    let user2_id = Ulid::new().to_string();
    let user2 = json!({
        "id": user2_id,
        "name": "User Two",
        "email": "user.two@example.com"
    });
    app.clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/users")
                .header("content-type", "application/json")
                .body(Body::from(user2.to_string()))
                .unwrap(),
        )
        .await
        .unwrap();

    // Try to update user 1 with user 2's ID (which should fail)
    let update_user1 = json!({
        "id": user2_id,
        "name": "Updated User One",
        "email": "updated.user.one@example.com"
    });
    let response = app
        .oneshot(
            Request::builder()
                .method("PUT")
                .uri(&format!("/users/{}", user1_id))
                .header("content-type", "application/json")
                .body(Body::from(update_user1.to_string()))
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::BAD_REQUEST);

    let body = to_bytes(response.into_body(), 1024).await.unwrap();
    let error: serde_json::Value = serde_json::from_slice(&body).unwrap();

    assert_eq!(error, json!({"error": "New user ID already exists"}));
}