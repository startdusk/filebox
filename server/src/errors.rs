use std::{io, string};

use actix_web::{http::StatusCode, HttpResponse, ResponseError};
use validator::ValidationErrors;

use crate::api::ErrorResponse;

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

    #[error("Ip allow error")]
    IpAllowerError(i32),

    #[error("Unknown error")]
    Unknown,

    #[error("Redis error")]
    RedisError(#[from] anyhow::Error),

    #[error("Redis send command error")]
    RedisSendCommandError(String),

    #[error("Deserialize json error")]
    DeserializeJsonError(#[from] serde_json::Error),

    #[error("Parse get reids value to string error")]
    ParseGetRedisValue(#[from] string::FromUtf8Error),
}

impl Error {
    fn error_response(&self) -> String {
        match self {
            Error::IpAllowerError(limit) => {
                format!("今日文件口令错误已达 {limit} 次, 请明天再访问")
            }
            Error::InvalidCode(msg) => msg.to_string(),
            Error::ValidateArgsError(_) | Error::InputValidateError(_) => {
                "input validate error".to_string()
            }

            Error::InvalidFileType(err) => format!("invalid file type: {err}"),
            Error::NotFound => "not found".to_string(),
            Error::IOError(_)
            | Error::MultipartError(_)
            | Error::DeserializeJsonError(_)
            | Error::RedisError(_)
            | Error::RedisSendCommandError(_)
            | Error::DbError(_)
            | Error::ParseGetRedisValue(_)
            | Error::Unknown => "internal server error".to_string(),
        }
    }

    fn name(&self) -> String {
        match self {
            Error::ValidateArgsError(_) => "VALIDATE_ARGS_ERROR".to_string(),
            Error::InvalidCode(_) => "INVALID_CODE".to_string(),
            Error::InvalidFileType(_) => "INVALID_FILE_TYPE".to_string(),
            Error::InputValidateError(_) => "INPUT_VALIDATE_ERROR".to_string(),
            Error::NotFound => "NOT_FOUND".to_string(),
            Error::IpAllowerError(_) => "IP_ALLOWER_ERROR".to_string(),
            Error::IOError(_) => "IO_ERROR".to_string(),
            Error::DbError(_) => "DB_ERROR".to_string(),
            Error::Unknown => "UNKNOWN".to_string(),
            Error::MultipartError(_) => "MULTIPART_ERROR".to_string(),
            Error::RedisError(_) => "REDIS_ERROR".to_string(),
            Error::DeserializeJsonError(_) => "DESERIALIZE_JSON_ERROR".to_string(),
            Error::ParseGetRedisValue(_) => "PARSE_GET_REDIS_VALUE".to_string(),
            Error::RedisSendCommandError(_) => "REDIS_SEND_COMMAND_ERROR".to_string(),
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

impl ResponseError for Error {
    fn status_code(&self) -> actix_web::http::StatusCode {
        match self {
            Error::ValidateArgsError(_)
            | Error::InvalidCode(_)
            | Error::InvalidFileType(_)
            | Error::InputValidateError(_) => StatusCode::BAD_REQUEST,
            Error::NotFound => StatusCode::NOT_FOUND,
            Error::IpAllowerError(_) => StatusCode::FORBIDDEN,
            Error::IOError(_)
            | Error::DbError(_)
            | Error::Unknown
            | Error::DeserializeJsonError(_)
            | Error::MultipartError(_)
            | Error::ParseGetRedisValue(_)
            | Error::RedisSendCommandError(_)
            | Error::RedisError(_) => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }

    fn error_response(&self) -> actix_web::HttpResponse<actix_web::body::BoxBody> {
        HttpResponse::build(self.status_code()).json(ErrorResponse {
            message: self.error_response(),
            code: self.status_code().as_u16(),
            error: self.name(),
        })
    }
}
