use sqlx::types::chrono::NaiveDateTime;

#[derive(Debug, Default)]
pub struct AddFilebox {
    pub code: String,
    pub name: String,
    pub size: i64,
    pub file_type: FileType,
    pub text: String,
    pub file_path: String,
    pub created_at: NaiveDateTime,
    pub expired_at: NaiveDateTime,
}

pub struct UpdateFilebox {
    pub code: String,
    pub used_at: NaiveDateTime,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Filebox {
    pub id: i64,
    pub code: String,
    pub name: String,
    pub size: i64,
    pub file_type: FileType,
    pub text: String,
    pub file_path: String,
    pub created_at: NaiveDateTime,
    pub expired_at: NaiveDateTime,
    pub used_at: Option<NaiveDateTime>,
}

impl Filebox {
    pub fn has_taken(&self) -> bool {
        self.used_at.is_some()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, sqlx::Type)]
#[sqlx(type_name = "file_type", rename_all = "lowercase")]
pub enum FileType {
    Text,
    File,
}

impl Default for FileType {
    fn default() -> Self {
        FileType::Text
    }
}

impl From<FileType> for String {
    fn from(value: FileType) -> Self {
        match value {
            FileType::File => "file".to_string(),
            FileType::Text => "text".to_string(),
        }
    }
}
