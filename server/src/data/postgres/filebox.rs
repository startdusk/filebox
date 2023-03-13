use chrono::Local;
use sqlx::PgPool;

use crate::{
    errors::Error,
    models::filebox::{AddFilebox, Filebox},
};

pub async fn get_filebox_db(pool: &PgPool, code: String) -> Result<Filebox, Error> {
    let filebox: Filebox = sqlx::query_as(
        r#"
			SELECT * FROM filebox WHERE code = $1
		"#,
    )
    .bind(code)
    .fetch_one(pool)
    .await?;

    Ok(filebox)
}

pub async fn delete_expired_filebox_db(pool: &PgPool) -> Result<Vec<Filebox>, Error> {
    let now = Local::now().naive_local();

    let filebox_vec: Vec<Filebox> = sqlx::query_as(
        r#"
		DELETE FROM filebox WHERE expired_at <= $1 OR used_at IS NOT NULL RETURNING *
	"#,
    )
    .bind(now)
    .fetch_all(pool)
    .await?;

    Ok(filebox_vec)
}

pub async fn add_new_filebox_db(pool: &PgPool, filebox: AddFilebox) -> Result<Filebox, Error> {
    let file_type: String = filebox.file_type.into();
    let new_filebox: Filebox = sqlx::query_as(
        r#"
			INSERT INTO filebox (
				code, 
				name,
				size,
				file_type,
				text,
                file_path,
				created_at,
				expired_at
			) VALUES (
				$1, $2, $3, $4::file_type, $5, $6, $7, $8
			) RETURNING *
		"#,
    )
    .bind(filebox.code)
    .bind(filebox.name)
    .bind(filebox.size)
    .bind(file_type)
    .bind(filebox.text)
    .bind(filebox.file_path)
    .bind(filebox.created_at)
    .bind(filebox.expired_at)
    .fetch_one(pool)
    .await?;

    Ok(new_filebox)
}

pub async fn update_filebox_db(pool: &PgPool, code: String) -> Result<Filebox, Error> {
    let now = Local::now().naive_local();
    let filebox: Filebox = sqlx::query_as(
        r#"
		UPDATE filebox SET used_at = $1 WHERE code = $2 AND used_at IS NULL RETURNING *
	"#,
    )
    .bind(now)
    .bind(code)
    .fetch_one(pool)
    .await?;

    Ok(filebox)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::filebox::FileType;

    use std::ops::Add;

    use crate::test_utils::get_tdb;
    use chrono::{Duration, Local};

    #[actix_rt::test]
    async fn filebox_lifecycle() {
        let tdb = get_tdb();
        let pool = tdb.get_pool().await;

        // 1.insert a new filebox
        let code = "12345".to_string();
        let now = Local::now().naive_local();
        let filebox = AddFilebox {
            code: code.clone(),
            name: "test".to_string(),
            file_type: FileType::Text,
            text: "21123".to_string(),
            created_at: now,
            expired_at: now.add(Duration::days(7)),
            ..Default::default()
        };
        let new_filebox = add_new_filebox_db(&pool, filebox.clone()).await.unwrap();
        assert_eq!(filebox.code, new_filebox.code);
        assert_eq!(filebox.name, new_filebox.name);
        assert_eq!(filebox.file_type, new_filebox.file_type);
        assert_eq!(filebox.text, new_filebox.text);
        assert_eq!(
            filebox.created_at.timestamp(),
            new_filebox.created_at.timestamp()
        );
        assert_eq!(
            filebox.expired_at.timestamp(),
            new_filebox.expired_at.timestamp()
        );

        // 2.get the filebox
        let get_filebox = get_filebox_db(&pool, code.clone()).await.unwrap();
        assert_eq!(new_filebox, get_filebox);

        // 3.update the filebox set used
        let get_update_filebox = update_filebox_db(&pool, code.clone()).await.unwrap();

        assert!(get_update_filebox.has_taken());

        // 4.delete the used filebox
        let filebox_vec = delete_expired_filebox_db(&pool).await.unwrap();
        assert_eq!(filebox_vec.len(), 1);
        let resp = get_filebox_db(&pool, code.clone()).await;
        assert!(resp.is_err());
    }
}
