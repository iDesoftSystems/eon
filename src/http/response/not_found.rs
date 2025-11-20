use crate::http::{ProblemDetails, ResourceId};
use axum::Json;
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};

/// Represents a 404 Not Found response.
///
/// This struct wraps a `ResourceId` and converts it into a `ProblemDetails` response
/// with a descriptive message.
pub struct ResourceNotFound(pub ResourceId);

impl IntoResponse for ResourceNotFound {
    fn into_response(self) -> Response {
        let details =
            ProblemDetails::from(format!("Resource not found with identifier: {}", self.0));

        (StatusCode::NOT_FOUND, Json(details)).into_response()
    }
}
