use rocket::http::Status;
use rocket::response::{self, Responder};
use rocket::serde::json::Json;
use rocket::Request;
use serde::Serialize;
use thiserror::Error;

#[derive(Serialize)]
pub struct ErrorResponse {
    pub error: String,
    pub message: String,
}

#[derive(Error, Debug)]
pub enum Error {
    #[error("Bad request: {0}")]
    BadRequest(String),

    #[error("Unauthorized: {0}")]
    Unauthorized(String),

    #[error("Not found: {0}")]
    NotFound(String),

    #[error("Validation error: {0}")]
    ValidationError(String),

    #[error("Database error: {0}")]
    DatabaseError(String),

    #[error("Internal error: {0}")]
    Internal(String),
}

impl Error {
    fn status(&self) -> Status {
        match self {
            Error::BadRequest(_) | Error::ValidationError(_) => Status::BadRequest,
            Error::Unauthorized(_) => Status::Unauthorized,
            Error::NotFound(_) => Status::NotFound,
            Error::DatabaseError(_) | Error::Internal(_) => Status::InternalServerError,
        }
    }
}

impl<'r, 'o: 'r> Responder<'r, 'o> for Error {
    fn respond_to(self, req: &'r Request<'_>) -> response::Result<'o> {
        let status = self.status();
        let body = ErrorResponse {
            error: status.reason_lossy().to_string(),
            message: self.to_string(),
        };
        (status, Json(body)).respond_to(req)
    }
}

impl From<mongodb::error::Error> for Error {
    fn from(e: mongodb::error::Error) -> Self {
        Error::DatabaseError(e.to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bad_request_status() {
        let error = Error::BadRequest("invalid input".to_string());
        assert_eq!(error.status(), Status::BadRequest);
    }

    #[test]
    fn test_unauthorized_status() {
        let error = Error::Unauthorized("invalid token".to_string());
        assert_eq!(error.status(), Status::Unauthorized);
    }

    #[test]
    fn test_not_found_status() {
        let error = Error::NotFound("resource missing".to_string());
        assert_eq!(error.status(), Status::NotFound);
    }

    #[test]
    fn test_validation_error_status() {
        let error = Error::ValidationError("invalid field".to_string());
        assert_eq!(error.status(), Status::BadRequest);
    }

    #[test]
    fn test_database_error_status() {
        let error = Error::DatabaseError("connection failed".to_string());
        assert_eq!(error.status(), Status::InternalServerError);
    }

    #[test]
    fn test_internal_error_status() {
        let error = Error::Internal("unexpected".to_string());
        assert_eq!(error.status(), Status::InternalServerError);
    }

    #[test]
    fn test_error_display() {
        let error = Error::BadRequest("test message".to_string());
        assert_eq!(error.to_string(), "Bad request: test message");
    }

    #[test]
    fn test_error_response_serialization() {
        let response = ErrorResponse {
            error: "Bad Request".to_string(),
            message: "invalid input".to_string(),
        };
        let json = serde_json::to_string(&response).unwrap();
        assert!(json.contains("Bad Request"));
        assert!(json.contains("invalid input"));
    }
}
