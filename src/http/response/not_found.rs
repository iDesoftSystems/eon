use crate::http::{ProblemDetails, ResourceId};
use axum::Json;
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};

pub struct ResourceNotFound(pub ResourceId);

impl IntoResponse for ResourceNotFound {
    fn into_response(self) -> Response {
        let details =
            ProblemDetails::from(format!("Resource not found with identifier: {}", self.0));

        (StatusCode::NOT_FOUND, Json(details)).into_response()
    }
}
