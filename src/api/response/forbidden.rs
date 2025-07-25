use crate::types::problem::ProblemDetails;
use axum::Json;
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};

pub struct Forbidden;

impl IntoResponse for Forbidden {
    fn into_response(self) -> Response {
        (
            StatusCode::FORBIDDEN,
            Json(ProblemDetails::from("Forbidden".to_string())),
        )
            .into_response()
    }
}
