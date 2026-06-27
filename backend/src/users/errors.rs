use axum::http::StatusCode;
use axum::response::IntoResponse;

#[derive(Debug, thiserror::Error)]
pub enum AuthError {
    #[error(transparent)]
    HttpError(#[from] reqwest::Error),
    #[error(transparent)]
    JoseError(#[from] josekit::JoseError),
    #[error(transparent)]
    JsonError(#[from] serde_json::Error),
    #[error("unauthorized")]
    Unauthorized,
}

impl IntoResponse for AuthError {
    fn into_response(self) -> axum::response::Response {
        match &self {
            AuthError::HttpError(error) => {
                tracing::error!(?error, "something went wrong fetchin data");
            }
            AuthError::JoseError(jose_error) => {
                tracing::error!(?jose_error, "something went wrong dealing with jwt/jwk");
            }
            AuthError::JsonError(error) => {
                println!("{error:?}");
                tracing::error!(
                    ?error,
                    "something went wrong serializing/deserializing json"
                );
            }
            AuthError::Unauthorized => {
                tracing::error!("unauthorized");
            }
        };

        let status = match &self {
            AuthError::Unauthorized => StatusCode::UNAUTHORIZED,
            AuthError::HttpError(_) => StatusCode::INTERNAL_SERVER_ERROR,
            AuthError::JoseError(_) | AuthError::JsonError(_) => StatusCode::BAD_REQUEST,
        };

        (status, self.to_string()).into_response()
    }
}
