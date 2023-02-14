use std::io;

use actix_web::{http::StatusCode, HttpResponse, ResponseError};
use serde::Serialize;
use validator::ValidationErrors;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Database error")]
    DbError(sqlx::Error),

    #[error("Invalid code: {0}")]
    InvalidCode(String),

    #[error("Invalid file type: {0}")]
    InvalidFileType(String),

    #[error("Input validate error")]
    InputValidateError(#[from] ValidationErrors),

    #[error("Upload file error")]
    MultipartError(#[from] actix_easy_multipart::Error),

    #[error("IO error")]
    IOError(#[from] io::Error),

    #[error("Validate error: {0}")]
    ValidateArgsError(String),

    #[error("No file box found by the given condition")]
    NotFound,

    #[error("Unknown error")]
    Unknown,
}

impl Error {
    fn error_response(&self) -> String {
        match self {
            Error::InvalidCode(msg) => msg.to_string(),
            Error::ValidateArgsError(_) | Error::InputValidateError(_) => {
                "input validate error".to_string()
            }

            Error::InvalidFileType(err) => format!("invalid file type: {}", err),
            Error::NotFound => "not found".to_string(),
            Error::IOError(_) | Error::MultipartError(_) | Error::DbError(_) | Error::Unknown => {
                "internal server error".to_string()
            }
        }
    }
}

impl From<sqlx::Error> for Error {
    fn from(e: sqlx::Error) -> Self {
        match e {
            sqlx::Error::Database(e) => Error::DbError(sqlx::Error::Database(e)),
            sqlx::Error::RowNotFound => Error::NotFound,
            _ => Error::DbError(e),
        }
    }
}

#[derive(Debug, Serialize)]
pub struct ErrorResponse {
    pub message: String,
}

impl ResponseError for Error {
    fn status_code(&self) -> actix_web::http::StatusCode {
        match self {
            Error::ValidateArgsError(_)
            | Error::InvalidCode(_)
            | Error::InvalidFileType(_)
            | Error::InputValidateError(_) => StatusCode::BAD_REQUEST,
            Error::NotFound => StatusCode::NOT_FOUND,

            Error::IOError(_) | Error::DbError(_) | Error::Unknown | Error::MultipartError(_) => {
                StatusCode::INTERNAL_SERVER_ERROR
            }
        }
    }

    fn error_response(&self) -> actix_web::HttpResponse<actix_web::body::BoxBody> {
        HttpResponse::build(self.status_code()).json(ErrorResponse {
            message: self.error_response(),
        })
    }
}
