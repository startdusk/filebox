use atomic_refcell::AtomicRefCell;
use dashmap::DashMap;
use std::sync::Arc;

use actix_easy_multipart::{tempfile::Tempfile, text::Text, MultipartForm};
use serde::{
    de::{self, Unexpected},
    Deserialize, Serialize,
};
use validator::{Validate, ValidationError, ValidationErrors};

use crate::models::filebox::{FileType, Filebox};

#[derive(Debug, MultipartForm)]
pub struct CreateFileboxRequest {
    pub name: Text<String>,
    pub text: Option<Text<String>>,
    pub duration_day: Text<u8>,
    pub file_type: Text<FileboxFileType>,
    pub file: Option<Tempfile>,
}

impl Validate for CreateFileboxRequest {
    fn validate(&self) -> Result<(), validator::ValidationErrors> {
        let mut errors = ValidationErrors::new();

        if self.name.len() > 50 {
            errors.add("name", ValidationError::new("name more than 50 characters"));
        }

        if *self.duration_day == 0 || *self.duration_day >= 30 {
            errors.add(
                "duration_day",
                ValidationError::new("duration_day over scope"),
            );
        }

        match *self.file_type {
            FileboxFileType::Text => {
                if let Some(text) = &self.text {
                    if text.len() == 0 || text.len() > 2000 {
                        errors.add("text", ValidationError::new("text over scope"));
                    }
                } else {
                    errors.add("text", ValidationError::new("text empty"));
                }

                if !errors.is_empty() {
                    return Err(errors);
                }

                Ok(())
            }
            FileboxFileType::File => {
                if self.file.is_none() {
                    errors.add("file", ValidationError::new("file empty"));
                };

                if !errors.is_empty() {
                    return Err(errors);
                }

                Ok(())
            }
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetFileboxResponse {
    pub id: i64,
    pub code: String,
    pub name: String,
    pub file_type: FileboxFileType,
    pub created_at: i64,
    pub expired_at: i64,
    pub used_at: Option<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TakeTextResponse {
    pub id: i64,
    pub code: String,
    pub name: String,
    pub file_type: FileboxFileType,
    pub text: String,
    pub created_at: i64,
    pub expired_at: i64,
    pub used_at: i64,
}

#[derive(Debug, Clone, Serialize)]
pub struct CreateFileboxResponse {
    pub id: i64,
    pub code: String,
    pub name: String,
    pub file_type: FileboxFileType,
    pub created_at: i64,
    pub expired_at: i64,
}

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
pub enum FileboxFileType {
    Text = 1,
    File = 2,
}

#[derive(Debug, Clone, Serialize)]
pub struct HealthCheckResponse {
    pub message: String,
    pub health_check_count: u64,
}

impl From<&FileboxFileType> for u8 {
    fn from(v: &FileboxFileType) -> Self {
        match v {
            FileboxFileType::Text => 1,
            FileboxFileType::File => 2,
        }
    }
}

// https://serde.rs/impl-serializer.html
impl Serialize for FileboxFileType {
    fn serialize<S>(&self, s: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        s.serialize_u8(self.into())
    }
}

// 实现Deserialize参考: https://damad.be/joost/blog/rust-serde-deserialization-of-an-enum-variant.html
impl<'de> Deserialize<'de> for FileboxFileType {
    fn deserialize<D>(d: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let num = u8::deserialize(d)?;
        match num {
            1 => Ok(FileboxFileType::Text),
            2 => Ok(FileboxFileType::File),
            // TODO: 修改为指定的json序列化错误
            // 目前提示为：Json deserialize error: invalid value: integer `3`, expected 1 or 2 at line 3 column 1
            _ => Err(de::Error::invalid_value(
                Unexpected::Unsigned(num as u64),
                &"1 or 2",
            )),
        }
    }
}

impl From<FileboxFileType> for FileType {
    fn from(v: FileboxFileType) -> Self {
        match v {
            FileboxFileType::Text => FileType::Text,
            FileboxFileType::File => FileType::File,
        }
    }
}

impl From<FileType> for FileboxFileType {
    fn from(v: FileType) -> Self {
        match v {
            FileType::File => FileboxFileType::File,
            FileType::Text => FileboxFileType::Text,
        }
    }
}

impl From<Filebox> for GetFileboxResponse {
    fn from(v: Filebox) -> Self {
        Self {
            id: v.id,
            code: v.code,
            name: v.name,
            file_type: v.file_type.into(),
            created_at: v.created_at.timestamp(),
            expired_at: v.expired_at.timestamp(),
            used_at: v.used_at.map(|used_at| used_at.timestamp()),
        }
    }
}

impl From<Filebox> for TakeTextResponse {
    fn from(v: Filebox) -> Self {
        Self {
            id: v.id,
            code: v.code,
            name: v.name,
            text: v.text,
            file_type: v.file_type.into(),
            created_at: v.created_at.timestamp(),
            expired_at: v.expired_at.timestamp(),
            used_at: v.used_at.unwrap().timestamp(),
        }
    }
}

impl From<Filebox> for CreateFileboxResponse {
    fn from(v: Filebox) -> Self {
        Self {
            id: v.id,
            code: v.code,
            name: v.name,
            file_type: v.file_type.into(),
            created_at: v.created_at.timestamp(),
            expired_at: v.expired_at.timestamp(),
        }
    }
}

#[derive(Debug, Clone, Serialize)]
pub struct ErrorResponse {
    pub message: String,
}

pub type IPMap = Arc<DashMap<String, AtomicRefCell<IPInfo>>>;

#[derive(Debug)]
pub struct IPInfo {
    pub count: i32,
    pub expired_at: i64,
}

impl IPInfo {
    pub fn new(count: i32, expired_at: i64) -> Self {
        Self { count, expired_at }
    }
}
