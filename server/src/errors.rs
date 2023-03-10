use std::{io, string};

use actix_web::{http::StatusCode, HttpResponse, ResponseError};
use chrono::Local;
use validator::ValidationErrors;

use crate::api::{ErrorResponse, DATE_FORMAT, IP_UPLOAD_LIMIT_HEADER, IP_VISIT_ERROR_LIMIT_HEADER};

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

    #[error("Ip visit error limit")]
    IpVisitErrorLimit(i32),

    #[error("Ip upload limit")]
    IpUploadLimit(i32),

    #[error("Unknown error")]
    Unknown,

    #[error("Redis error")]
    RedisError(#[from] anyhow::Error),

    #[error("Actix web error: {0}")]
    ActixWebError(#[from] actix_web::Error),

    #[error("Redis send command error")]
    RedisSendCommandError(String),

    #[error("Deserialize json error")]
    DeserializeJsonError(#[from] serde_json::Error),

    #[error("Parse get reids value to string error")]
    ParseGetRedisValue(#[from] string::FromUtf8Error),
}

impl Error {
    pub fn to_response(&self) -> HttpResponse {
        self.error_response()
    }
    fn error_message(&self) -> String {
        match self {
            Error::IpVisitErrorLimit(limit) => {
                format!("今日文件口令错误已达 {limit} 次, 请明天再访问")
            }
            Error::IpUploadLimit(limit) => {
                format!("今日文件上传已达 {limit} 次, 请明天再上传")
            }
            Error::InvalidCode(msg) => msg.to_string(),
            Error::ValidateArgsError(_) | Error::InputValidateError(_) => {
                "input validate error".to_string()
            }

            Error::InvalidFileType(err) => format!("invalid file type: {err}"),
            Error::NotFound => "not found".to_string(),
            Error::ActixWebError(_)
            | Error::IOError(_)
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
            Error::IpVisitErrorLimit(_) => "IP_VISIT_ERROR_LIMIT".to_string(),
            Error::IpUploadLimit(_) => "IP_UPLOAD_LIMIT".to_string(),
            Error::IOError(_) => "IO_ERROR".to_string(),
            Error::DbError(_) => "DB_ERROR".to_string(),
            Error::Unknown => "UNKNOWN".to_string(),
            Error::MultipartError(_) => "MULTIPART_ERROR".to_string(),
            Error::RedisError(_) => "REDIS_ERROR".to_string(),
            Error::DeserializeJsonError(_) => "DESERIALIZE_JSON_ERROR".to_string(),
            Error::ParseGetRedisValue(_) => "PARSE_GET_REDIS_VALUE".to_string(),
            Error::RedisSendCommandError(_) => "REDIS_SEND_COMMAND_ERROR".to_string(),
            Error::ActixWebError(_) => "ACTIX_WEB_ERROR".to_string(),
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

            Error::IpVisitErrorLimit(_) | Error::IpUploadLimit(_) => StatusCode::FORBIDDEN,

            Error::ActixWebError(_)
            | Error::IOError(_)
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
        let mut builder = HttpResponse::build(self.status_code());
        let now = Local::now().format(DATE_FORMAT);
        let time_format_str: &str = &now.to_string();
        let builder = match self {
            Error::IpUploadLimit(_) => {
                builder.append_header((IP_UPLOAD_LIMIT_HEADER, time_format_str))
            }
            Error::IpVisitErrorLimit(_) => {
                builder.append_header((IP_VISIT_ERROR_LIMIT_HEADER, time_format_str))
            }
            _ => &mut builder,
        };
        builder.json(ErrorResponse {
            message: self.error_message(),
            code: self.status_code().as_u16(),
            error: self.name(),
        })
    }
}
