use axum::{
    body::{Body, to_bytes},
    http::{Request, StatusCode},
};
use hello_cargo::{app, User};
use serde_json::json;
use tower::ServiceExt; // Changed this line

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