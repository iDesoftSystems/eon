use crate::http::ProblemDetails;
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use axum::Json;

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
