use axum::{http::StatusCode, response::IntoResponse};

#[allow(dead_code)]
pub enum FluxError {
    DatabaseError(turso::Error),
    NotFound,
    CustomError(String),
}

impl IntoResponse for FluxError {
    fn into_response(self) -> axum::response::Response {
        match self {
            FluxError::DatabaseError(e) => {
                eprintln!("Database error: {:?}", e);
                (StatusCode::INTERNAL_SERVER_ERROR, "Database error.").into_response()
            }
            FluxError::NotFound => (StatusCode::NOT_FOUND, "Not found.").into_response(),
            FluxError::CustomError(message) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("idk - {message}"),
            )
                .into_response(),
        }
    }
}

// lets `?` auto-convert turso::Error into FluxError
impl From<turso::Error> for FluxError {
    fn from(e: turso::Error) -> Self {
        FluxError::DatabaseError(e)
    }
}
