use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use axum::Json;

use serde::Serialize;

#[derive(Serialize)]
pub struct Created<T>
where
    T: Serialize,
{
    pub id: T,
}

impl<T> Created<T>
where
    T: Serialize,
{
    pub fn new(id: T) -> Self {
        Self { id }
    }
}

impl<T> IntoResponse for Created<T>
where
    T: Serialize,
{
    fn into_response(self) -> Response {
        (StatusCode::CREATED, Json(self)).into_response()
    }
}
