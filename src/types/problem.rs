use serde::Serialize;

#[derive(Serialize)]
pub struct ProblemDetails {
    pub detail: String,
    pub errors: Vec<Field>,
}

#[derive(Serialize)]
pub struct Field {
    pub field: String,
    pub reason: String,
    pub code: String,
}

impl Field {
    pub fn new(field: &str, reason: &str, code: &str) -> Self {
        Self {
            field: field.to_string(),
            reason: reason.to_string(),
            code: code.to_string(),
        }
    }
}

impl From<String> for ProblemDetails {
    fn from(value: String) -> Self {
        Self {
            detail: value,
            errors: vec![],
        }
    }
}

impl From<Vec<Field>> for ProblemDetails {
    fn from(value: Vec<Field>) -> Self {
        let first_reason = value
            .first()
            .map(|field| field.reason.to_string())
            .unwrap_or("Your request parameters didn't pass validation".into());

        Self {
            detail: first_reason,
            errors: value,
        }
    }
}
