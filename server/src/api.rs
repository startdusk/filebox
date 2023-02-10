use serde::{
    de::{self, Unexpected},
    Deserialize, Serialize,
};
use validator::Validate;

use crate::models::filebox::{FileType, Filebox};

#[derive(Debug, Clone, Deserialize, Validate)]
pub struct CreateFileboxRequest {
    pub name: String,
    pub text: Option<String>,
    pub durations_day: u8,
    pub file_type: FileboxFileType,
}

/// GetFileboxResp 获取文件柜信息
#[derive(Debug, Clone, Serialize)]
pub struct GetFileboxResponse {
    pub id: i64,
    pub code: String,
    pub name: String,
    pub size: i64,
    pub file_type: FileboxFileType,
    pub text: String,
    pub file_path: String,
    pub created_at: i64,
    pub expired_at: i64,
    pub used_at: Option<i64>,
}

/// CreateFileboxResp 返回创建文件柜信息
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
            size: v.size,
            file_path: v.file_path,
            text: v.text,
            file_type: v.file_type.into(),
            created_at: v.created_at.timestamp(),
            expired_at: v.expired_at.timestamp(),
            used_at: v.used_at.map(|used_at| used_at.timestamp()),
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
