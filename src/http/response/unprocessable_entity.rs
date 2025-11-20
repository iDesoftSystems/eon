use crate::http::ProblemDetails;
use axum::Json;
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};

/// Represents a 422 Unprocessable Entity response.
///
/// This struct wraps a message string and converts it into a `ProblemDetails` response.
pub struct UnprocessableEntity(pub String);

impl IntoResponse for UnprocessableEntity {
    fn into_response(self) -> Response {
        (
            StatusCode::UNPROCESSABLE_ENTITY,
            Json(ProblemDetails::from(self.0)),
        )
            .into_response()
    }
}
