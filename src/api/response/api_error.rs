use crate::api::response::{
    BadRequest, EntityCodeNotFound, EntityIdNotFound, Forbidden, InternalServer,
    UnprocessableEntity,
};
use axum::response::{IntoResponse, Response};
use validator::ValidationErrors;

#[derive(Debug, thiserror::Error)]
pub enum ApiError {
    #[error("Internal error: {0}")]
    Internal(String),

    #[error("forbidden access for user with id: {0}")]
    Forbidden(i64),

    #[cfg(feature = "orm")]
    #[error(transparent)]
    DbError(#[from] sea_orm::DbErr),

    #[error(transparent)]
    Validation(#[from] ValidationErrors),

    #[error("Entity not found with id: {0}")]
    EntityIdNotFound(i64),

    #[error("Entity not found with code: {0}")]
    EntityCodeNotFound(String),

    #[error("{0}")]
    Message(String),

    #[error("{0}")]
    MessageStr(&'static str),
}

impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        match self {
            #[cfg(feature = "orm")]
            ApiError::DbError(err) => db_err_into_response(err),
            ApiError::Internal(msg) => {
                tracing::error!("Internal error: {msg}");
                InternalServer.into_response()
            }
            ApiError::Forbidden(id) => {
                tracing::error!("Forbidden access for user with: {id}");
                Forbidden.into_response()
            }
            ApiError::Validation(errs) => BadRequest(errs).into_response(),
            ApiError::EntityIdNotFound(id) => EntityIdNotFound(id).into_response(),
            ApiError::EntityCodeNotFound(code) => EntityCodeNotFound(code).into_response(),
            ApiError::Message(msg) => UnprocessableEntity(msg).into_response(),
            ApiError::MessageStr(msg) => UnprocessableEntity(msg.to_string()).into_response(),
        }
    }
}

#[cfg(feature = "orm")]
fn db_err_into_response(err: sea_orm::DbErr) -> Response {
    tracing::error!("DbErr: {:?}", err);

    match err {
        sea_orm::DbErr::Exec(sea_orm::RuntimeErr::SqlxError(sea_orm::sqlx::Error::Database(
            database_err,
        ))) => match database_err.kind() {
            sea_orm::sqlx::error::ErrorKind::UniqueViolation => {
                crate::api::response::Conflict.into_response()
            }
            sea_orm::sqlx::error::ErrorKind::ForeignKeyViolation => {
                crate::api::response::UnprocessableEntity(
                    "Cannot delete or update record because it is referenced by another record."
                        .to_string(),
                )
                .into_response()
            }
            _ => crate::api::response::InternalServer.into_response(),
        },
        _ => crate::api::response::InternalServer.into_response(),
    }
}
