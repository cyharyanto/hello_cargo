use axum::{
    body::{Body, to_bytes},
    http::{Request, StatusCode},
};
use hello_cargo::{app, User};
use serde_json::json;
use tower::ServiceExt;

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

#[tokio::test]
async fn test_create_duplicate_user() {
    let app = app();

    let new_user = json!({
        "id": 1,
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
    let app = app();

    // Create user 1
    let user1 = json!({
        "id": 1,
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
    let user2 = json!({
        "id": 2,
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

    // Try to update user 1 with id = 2 (which should fail)
    let update_user1 = json!({
        "id": 2,
        "name": "Updated User One",
        "email": "updated.user.one@example.com"
    });
    let response = app
        .oneshot(
            Request::builder()
                .method("PUT")
                .uri("/users/1")
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