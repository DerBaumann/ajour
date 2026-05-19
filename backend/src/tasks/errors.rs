use axum::{Json, http::StatusCode, response::IntoResponse};

#[derive(Debug, thiserror::Error)]
pub enum TaskError {
    #[error(transparent)]
    Validation(#[from] validator::ValidationErrors),

    #[error(transparent)]
    Database(#[from] sqlx::Error),

    #[error(transparent)]
    TimeParsing(#[from] time::Error),
}

impl IntoResponse for TaskError {
    fn into_response(self) -> axum::response::Response {
        match self {
            TaskError::Validation(validation_errors) => {
                (StatusCode::BAD_REQUEST, Json(validation_errors.errors())).into_response()
            }
            TaskError::Database(error) => {
                (StatusCode::INTERNAL_SERVER_ERROR, error.to_string()).into_response()
            }
            TaskError::TimeParsing(error) => {
                (StatusCode::BAD_REQUEST, error.to_string()).into_response()
            }
        }
    }
}
