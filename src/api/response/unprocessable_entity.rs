use crate::types::problem::ProblemDetails;
use axum::Json;
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};

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
