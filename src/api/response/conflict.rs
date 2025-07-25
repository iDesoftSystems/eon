use crate::types::problem::ProblemDetails;
use axum::Json;
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};

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
