use actix_web::{http::StatusCode, HttpResponse, ResponseError};

#[allow(dead_code)]
#[derive(thiserror::Error, Debug)]
pub enum ApiError {
    #[error("Unauthorized")]
    Unauthorized,
    #[error("Permission Denied")]
    PermissionDenied,
    #[error("Requested resource is not found")]
    NotFound,
    #[error("Failed to fetch data from discord.\n{0}")]
    DiscordClientError(#[from] reqwest::Error),
    #[error("Failed to fetch data: {0}")]
    DatabaseError(#[from] sqlx::Error),
    #[error("Failed to fetch data")]
    DeserializeFailed(#[from] serde::de::value::Error),
    #[error("{0}")]
    Internal(#[from] actix_web::error::Error),
    #[error("{0}")]
    Other(#[from] anyhow::Error),
}

#[derive(Serialize, Debug)]
struct ErrorResponse {
    code: u16,
    message: String,
}

impl ResponseError for ApiError {
    fn status_code(&self) -> StatusCode {
        match *self {
            ApiError::Unauthorized => StatusCode::UNAUTHORIZED,
            ApiError::PermissionDenied => StatusCode::FORBIDDEN,
            ApiError::NotFound => StatusCode::NOT_FOUND,
            ApiError::DiscordClientError(_) => StatusCode::INTERNAL_SERVER_ERROR,
            ApiError::DatabaseError(_) => StatusCode::INTERNAL_SERVER_ERROR,
            ApiError::DeserializeFailed(_) => StatusCode::INTERNAL_SERVER_ERROR,
            ApiError::Internal(_) => StatusCode::INTERNAL_SERVER_ERROR,
            ApiError::Other(_) => StatusCode::SERVICE_UNAVAILABLE,
        }
    }

    fn error_response(&self) -> HttpResponse {
        let status_code = self.status_code();
        let resp = ErrorResponse {
            code: status_code.as_u16(),
            message: self.to_string(),
        };
        HttpResponse::build(status_code).json(resp)
    }
}
