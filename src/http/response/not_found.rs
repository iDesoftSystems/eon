use crate::http::{ProblemDetails, ResourceId};
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use axum::Json;

pub struct ResourceNotFound(pub ResourceId);

impl IntoResponse for ResourceNotFound {
    fn into_response(self) -> Response {
        let details =
            ProblemDetails::from(format!("Resource not found with identifier: {}", self.0));

        (StatusCode::NOT_FOUND, Json(details)).into_response()
    }
}

pub struct EntityIdNotFound(pub i64);

pub struct EntityCodeNotFound(pub String);

impl IntoResponse for EntityIdNotFound {
    fn into_response(self) -> Response {
        (
            StatusCode::NOT_FOUND,
            Json(ProblemDetails::from(format!(
                "Entity not found with id: {}",
                self.0
            ))),
        )
            .into_response()
    }
}

impl IntoResponse for EntityCodeNotFound {
    fn into_response(self) -> Response {
        (
            StatusCode::NOT_FOUND,
            Json(ProblemDetails::from(format!(
                "Entity not found with code: {}",
                self.0
            ))),
        )
            .into_response()
    }
}
