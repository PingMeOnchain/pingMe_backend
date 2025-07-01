use axum::{body::Body, http::Request};
use tower::ServiceExt; // for `oneshot` method
use serde_json::json;
use pingMe_backend::{SubscriptionRequest, NotificationPreference, Channel, PreferenceUpdateRequest};

#[tokio::test]
async fn test_subscribe_and_preferences() {
    let pool = sqlx::sqlite::SqlitePoolOptions::new()
        .max_connections(1)
        .connect(":memory:")
        .await
        .unwrap();
    pingMe_backend::init_db(&pool).await.unwrap();
    let app = pingMe_backend::app(pool.clone());

    // Subscribe
    let req = SubscriptionRequest {
        event: "event1".to_string(),
        preferences: vec![NotificationPreference { channel: Channel::Email, enabled: true }],
    };
    let body = serde_json::to_vec(&req).unwrap();
    let response = app
        .clone()
        .oneshot(Request::builder()
            .method("POST")
            .uri("/subscribe/user1")
            .header("content-type", "application/json")
            .body(Body::from(body))
            .unwrap())
        .await
        .unwrap();
    assert_eq!(response.status(), 200);

    // Get preferences
    let response = app
        .clone()
        .oneshot(Request::builder()
            .method("GET")
            .uri("/preferences/user1/event1")
            .body(Body::empty())
            .unwrap())
        .await
        .unwrap();
    assert_eq!(response.status(), 200);

    // Update preferences
    let update = PreferenceUpdateRequest {
        preferences: vec![NotificationPreference { channel: Channel::Email, enabled: false }],
    };
    let body = serde_json::to_vec(&update).unwrap();
    let response = app
        .clone()
        .oneshot(Request::builder()
            .method("POST")
            .uri("/preferences/user1/event1")
            .header("content-type", "application/json")
            .body(Body::from(body))
            .unwrap())
        .await
        .unwrap();
    assert_eq!(response.status(), 200);

    // Unsubscribe
    let response = app
        .clone()
        .oneshot(Request::builder()
            .method("POST")
            .uri("/unsubscribe/user1/event1")
            .body(Body::empty())
            .unwrap())
        .await
        .unwrap();
    assert_eq!(response.status(), 204);
}
