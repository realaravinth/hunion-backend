use actix_http::ResponseBuilder;
use actix_web::{error::ResponseError, http::header, http::StatusCode, HttpResponse};
use diesel::result::Error as DBError;
use failure::Fail;
use serde::{Deserialize, Serialize};

use std::convert::From;

#[derive(Debug, PartialEq, Fail)]
#[cfg(not(tarpaulin_include))]
pub enum ServiceError {
    #[fail(display = "already answered")] //405
    AlreadyAnswered,
    #[fail(display = "invalid credentials")]
    AuthorizationRequired,
    #[fail(display = "internal error")] // 500
    InternalServerError,
    #[fail(display = "time over")] //408
    Timeout,
    #[fail(display = "too early")] //403
    TooEarly,
    #[fail(display = "Unable to connect to DB")]
    UnableToConnectToDb,
}

#[derive(Serialize, Deserialize)]
#[cfg(not(tarpaulin_include))]
struct ErrorToResponse {
    error: String,
}

impl ResponseError for ServiceError {
    fn error_response(&self) -> HttpResponse {
        ResponseBuilder::new(self.status_code())
            .set_header(header::CONTENT_TYPE, "application/json; charset=UTF-8")
            .json(ErrorToResponse {
                error: self.to_string(),
            })
    }

    fn status_code(&self) -> StatusCode {
        match *self {
            ServiceError::AlreadyAnswered => StatusCode::CONFLICT,
            ServiceError::AuthorizationRequired => StatusCode::UNAUTHORIZED,
            ServiceError::InternalServerError => StatusCode::INTERNAL_SERVER_ERROR,
            ServiceError::Timeout => StatusCode::FORBIDDEN,
            ServiceError::UnableToConnectToDb => StatusCode::INTERNAL_SERVER_ERROR,
            ServiceError::TooEarly => StatusCode::FORBIDDEN,
        }
    }
}

impl From<DBError> for ServiceError {
    fn from(_: DBError) -> ServiceError {
        // Right now we just care about UniqueViolation from diesel
        // But this would be helpful to easily map errors as our app grows
        ServiceError::InternalServerError
    }
}

impl From<std::time::SystemTimeError> for ServiceError {
    fn from(_: std::time::SystemTimeError) -> ServiceError {
        ServiceError::InternalServerError
    }
}
impl From<actix_http::Error> for ServiceError {
    fn from(_: actix_http::Error) -> ServiceError {
        ServiceError::InternalServerError
    }
}

pub type ServiceResult<V> = std::result::Result<V, crate::errors::ServiceError>;
