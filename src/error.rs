use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde_json::json;

#[derive(Debug, thiserror::Error)]
pub enum AppError {
    #[error("Not found")]
    NotFound,

    #[error("{0}")]
    BadRequest(String),

    // #[from] lets `?` convert sqlx::Error into AppError automatically
    #[error("Something went wrong")]
    Database(#[from] sqlx::Error),
}

// Teaches Axum how to turn an AppError into an HTTP response
impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let status = match &self {
            AppError::NotFound => StatusCode::NOT_FOUND,
            AppError::BadRequest(_) => StatusCode::BAD_REQUEST,
            AppError::Database(e) => {
                eprintln!("Database error: {e}"); // log the real error, hide it from the client
                StatusCode::INTERNAL_SERVER_ERROR
            }
        };

        (status, Json(json!({ "error": self.to_string() }))).into_response()
    }
}

// Shortcut so handlers can write AppResult<T> instead of Result<T, AppError>
pub type AppResult<T> = Result<T, AppError>;