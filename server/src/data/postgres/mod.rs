mod filebox;

pub use filebox::*;

use sqlx::{postgres::PgRow, types::chrono::NaiveDateTime, FromRow, Row};

use crate::models::filebox::{FileType, Filebox};

impl FromRow<'_, PgRow> for Filebox {
    fn from_row(row: &PgRow) -> Result<Self, sqlx::Error> {
        let id: i64 = row.get("id");
        let code: String = row.get("code");
        let name: String = row.get("name");
        let size: i64 = row.get("size");
        let file_path: String = row.get("file_path");
        let created_at: NaiveDateTime = row.get("created_at");
        let expired_at: NaiveDateTime = row.get("expired_at");
        let used_at: Option<NaiveDateTime> = row.get("used_at");
        let file_type: FileType = row.get("file_type");
        let text: String = row.get("text");
        Ok(Filebox {
            id,
            code,
            name,
            size,
            file_path,
            created_at,
            expired_at,
            used_at,
            file_type,
            text,
        })
    }
}
