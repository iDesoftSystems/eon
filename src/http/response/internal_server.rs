use crate::http::ProblemDetails;
use axum::Json;
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};

pub struct InternalServer;

impl IntoResponse for InternalServer {
    fn into_response(self) -> Response {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(ProblemDetails::from("Internal Server Error".to_string())),
        )
            .into_response()
    }
}
