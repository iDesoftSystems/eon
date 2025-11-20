use crate::http::ProblemDetails;
use axum::Json;
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};

/// Represents a 401 Unauthorized response.
pub struct Unauthorized;

impl IntoResponse for Unauthorized {
    fn into_response(self) -> Response {
        (
            StatusCode::UNAUTHORIZED,
            Json(ProblemDetails::from("Unauthorized".to_string())),
        )
            .into_response()
    }
}
