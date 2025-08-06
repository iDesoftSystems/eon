use crate::types::problem::ProblemDetails;
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use axum::Json;

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

pub struct ConflictWithMessage(pub String);

impl IntoResponse for ConflictWithMessage {
    fn into_response(self) -> Response {
        (StatusCode::CONFLICT, Json(ProblemDetails::from(self.0))).into_response()
    }
}
