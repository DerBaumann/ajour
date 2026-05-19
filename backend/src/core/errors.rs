use axum::{http::StatusCode, response::IntoResponse};

pub struct AnyhowError(anyhow::Error);

impl From<anyhow::Error> for AnyhowError {
    fn from(value: anyhow::Error) -> Self {
        Self(value)
    }
}

impl IntoResponse for AnyhowError {
    fn into_response(self) -> axum::response::Response {
        (StatusCode::INTERNAL_SERVER_ERROR, self.0.to_string()).into_response()
    }
}
