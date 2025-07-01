
use axum::{routing::{post, get}, Router, Json, extract::Path, http::StatusCode};
use axum::extract::State;
use sqlx::Row;
use std::sync::Arc;

// Use explicit type annotation for the handler state
type AppState = Arc<SqlitePool>;

fn app(pool: AppState) -> Router {
    Router::new()
        .route("/subscribe/:user_id", post(subscribe))
        .route("/unsubscribe/:user_id/:event", post(unsubscribe))
        .route("/preferences/:user_id/:event", get(get_preferences).post(update_preferences))
        .with_state(pool)
}
use serde::{Deserialize, Serialize};
use sqlx::{SqlitePool, sqlite::SqlitePoolOptions};
use std::net::SocketAddr;
use thiserror::Error;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum Channel {
    Email,
    Telegram,
    Webhook,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct NotificationPreference {
    pub channel: Channel,
    pub enabled: bool,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Subscription {
    pub user_id: String,
    pub event: String,
    pub preferences: Vec<NotificationPreference>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SubscriptionRequest {
    pub event: String,
    pub preferences: Vec<NotificationPreference>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PreferenceUpdateRequest {
    pub preferences: Vec<NotificationPreference>,
}

#[derive(Debug, Error)]
pub enum ApiError {
    #[error("Database error: {0}")]
    Database(#[from] sqlx::Error),
    #[error("Invalid request: {0}")]
    InvalidRequest(String),
}

impl axum::response::IntoResponse for ApiError {
    fn into_response(self) -> axum::response::Response {
        let msg = self.to_string();
        let status = match self {
            ApiError::Database(_) => StatusCode::INTERNAL_SERVER_ERROR,
            ApiError::InvalidRequest(_) => StatusCode::BAD_REQUEST,
        };
        (status, Json(serde_json::json!({"error": msg}))).into_response()
    }
}

#[axum::debug_handler]
async fn subscribe(
    Path(user_id): Path<String>,
    State(pool): State<Arc<SqlitePool>>,
    Json(req): Json<SubscriptionRequest>,
) -> Result<Json<Subscription>, ApiError> {
    // Insert or update subscription in DB
    let preferences_json = serde_json::to_string(&req.preferences).unwrap();
    sqlx::query(
        "INSERT INTO subscriptions (user_id, event, preferences) VALUES (?, ?, ?) \
        ON CONFLICT(user_id, event) DO UPDATE SET preferences=excluded.preferences"
    )
    .bind(&user_id)
    .bind(&req.event)
    .bind(&preferences_json)
    .execute(pool.as_ref())
    .await?;
    Ok(Json(Subscription {
        user_id,
        event: req.event,
        preferences: req.preferences,
    }))
}

async fn unsubscribe(
    Path((user_id, event)): Path<(String, String)>,
    State(pool): State<Arc<SqlitePool>>,
) -> Result<StatusCode, ApiError> {
    let res = sqlx::query(
        "DELETE FROM subscriptions WHERE user_id = ? AND event = ?"
    )
    .bind(&user_id)
    .bind(&event)
    .execute(pool.as_ref())
    .await?;
    if res.rows_affected() == 0 {
        Err(ApiError::InvalidRequest("No subscription found".into()))
    } else {
        Ok(StatusCode::NO_CONTENT)
    }
}

async fn get_preferences(
    Path((user_id, event)): Path<(String, String)>,
    State(pool): State<Arc<SqlitePool>>,
) -> Result<Json<Subscription>, ApiError> {
    let rec = sqlx::query(
        "SELECT preferences FROM subscriptions WHERE user_id = ? AND event = ?"
    )
    .bind(&user_id)
    .bind(&event)
    .fetch_one(pool.as_ref())
    .await
    .map_err(|_| ApiError::InvalidRequest("Subscription not found".into()))?;
    let preferences: Vec<NotificationPreference> = serde_json::from_str(rec.try_get::<&str, _>("preferences").unwrap())
        .map_err(|_| ApiError::InvalidRequest("Invalid preferences format".into()))?;
    Ok(Json(Subscription {
        user_id,
        event,
        preferences,
    }))
}

async fn update_preferences(
    Path((user_id, event)): Path<(String, String)>,
    State(pool): State<Arc<SqlitePool>>,
    Json(req): Json<PreferenceUpdateRequest>,
) -> Result<Json<Subscription>, ApiError> {
    let preferences_json = serde_json::to_string(&req.preferences).unwrap();
    let res = sqlx::query(
        "UPDATE subscriptions SET preferences = ? WHERE user_id = ? AND event = ?"
    )
    .bind(&preferences_json)
    .bind(&user_id)
    .bind(&event)
    .execute(pool.as_ref())
    .await?;
    if res.rows_affected() == 0 {
        return Err(ApiError::InvalidRequest("Subscription not found".into()));
    }
    Ok(Json(Subscription {
        user_id,
        event,
        preferences: req.preferences,
    }))
}

pub async fn init_db(pool: &SqlitePool) -> Result<(), sqlx::Error> {
    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS subscriptions (
            user_id TEXT NOT NULL,
            event TEXT NOT NULL,
            preferences TEXT NOT NULL,
            PRIMARY KEY (user_id, event)
        )
        "#
    )
    .execute(pool)
    .await?;
    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), ApiError> {
    let pool = SqlitePoolOptions::new()
        .max_connections(5)
        .connect("sqlite://pingme.db").await?;
    init_db(&pool).await?;

    let pool = Arc::new(pool);
    let app = app(pool.clone());

    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));
    println!("Listening on {}", addr);
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app.into_make_service()).await.unwrap();
    Ok(())
}
