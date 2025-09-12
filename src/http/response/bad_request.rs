use crate::http::{Field, ProblemDetails};
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use axum::Json;
use std::borrow::Cow;
use validator::ValidationErrors;

pub struct BadRequest(pub ValidationErrors);

trait IntoFields {
    fn into_fields(self) -> Vec<Field>;
}

impl IntoResponse for BadRequest {
    fn into_response(self) -> Response {
        let fields = self.0.into_fields();

        (StatusCode::BAD_REQUEST, Json(ProblemDetails::from(fields))).into_response()
    }
}

impl IntoFields for ValidationErrors {
    fn into_fields(self) -> Vec<Field> {
        let mut fields = self
            .field_errors()
            .into_iter()
            .map(|error| {
                let default_message = Cow::from("Invalid information");
                let field_message = error.1[0].message.as_ref().unwrap_or(&default_message);

                Field::new(error.0.as_ref(), field_message, error.1[0].code.as_ref())
            })
            .collect::<Vec<Field>>();

        fields.sort_by(|a, b| a.field.to_lowercase().cmp(&b.field.to_lowercase()));

        fields
    }
}
