use crate::http::ProblemDetails;
use axum::Json;
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};

/// Represents a 409 Conflict response with a default message.
pub struct Conflict;

impl IntoResponse for Conflict {
    fn into_response(self) -> Response {
        (
            StatusCode::CONFLICT,
            Json(ProblemDetails::from("Conflict".to_string())),
        )
            .into_response()
    }
}

/// Represents a 409 Conflict response with a custom message.
pub struct ConflictWithMessage(pub String);

impl IntoResponse for ConflictWithMessage {
    fn into_response(self) -> Response {
        (StatusCode::CONFLICT, Json(ProblemDetails::from(self.0))).into_response()
    }
}
