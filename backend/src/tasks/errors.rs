use axum::{http::StatusCode, response::IntoResponse};

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
        match &self {
            TaskError::Validation(validation_errors) => {
                tracing::error!(?validation_errors, "validation_errors");
            }
            TaskError::Database(error) => {
                tracing::error!(?error, "database error");
            }
            TaskError::TimeParsing(error) => {
                tracing::error!(?error, "time error");
            }
        };

        let status = match &self {
            TaskError::Validation(_) | TaskError::TimeParsing(_) => StatusCode::BAD_REQUEST,
            TaskError::Database(_) => StatusCode::INTERNAL_SERVER_ERROR,
        };

        (status, self.to_string()).into_response()
    }
}
