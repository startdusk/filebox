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

pub async fn delete_expired_filebox_db(pool: &PgPool) -> Result<(), Error> {
    let now = Local::now().naive_local();
    let _ = sqlx::query(
        r#"
		DELETE FROM filebox WHERE expired_at < $1 OR used_at IS NOT NULL
	"#,
    )
    .bind(now)
    .execute(pool)
    .await?;

    Ok(())
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
				created_at,
				expired_at
			) VALUES (
				$1, $2, $3, $4::file_type, $5, $6, $7
			) RETURNING *
		"#,
    )
    .bind(filebox.code)
    .bind(filebox.name)
    .bind(filebox.size)
    .bind(file_type)
    .bind(filebox.text)
    .bind(filebox.created_at)
    .bind(filebox.expired_at)
    .fetch_one(pool)
    .await?;

    Ok(new_filebox)
}

pub async fn update_filebox_db(pool: &PgPool, code: String) -> Result<(), Error> {
    let now = Local::now().naive_local();
    let _ = sqlx::query(
        r#"
		UPDATE filebox SET used_at = $1 WHERE code = $2
	"#,
    )
    .bind(now)
    .bind(code)
    .execute(pool)
    .await?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::filebox::FileType;

    use std::{ops::Add, path::Path};

    use chrono::{Duration, Local};
    use sqlx_db_tester::TestPg;

    #[actix_rt::test]
    async fn filebox_lifecycle() {
        let tdb = get_tdb();
        let pool = tdb.get_pool().await;

        // 1.insert a new filebox
        let code = "12345".to_string();
        let now = Local::now().naive_local();
        let filebox = AddFilebox {
            code: code.clone(),
            name: "test.zip".to_string(),
            size: 123,
            file_type: FileType::File,
            file_path: "..///sdfsa".to_string(),
            text: "21123".to_string(),
            created_at: now,
            expired_at: now.add(Duration::days(7)),
        };
        let new_filebox = add_new_filebox_db(&pool, filebox).await.unwrap();

        // 2.get the filebox
        let get_filebox = get_filebox_db(&pool, code.clone()).await.unwrap();
        assert_eq!(new_filebox, get_filebox);

        // 3.update the filebox set used
        update_filebox_db(&pool, code.clone()).await.unwrap();

        let get_update_filebox = get_filebox_db(&pool, code.clone()).await.unwrap();
        assert!(get_update_filebox.has_taken());

        // 4.delete the used filebox
        delete_expired_filebox_db(&pool).await.unwrap();

        let resp = get_filebox_db(&pool, code.clone()).await;
        assert!(resp.is_err());
    }

    // private none test functions
    fn get_tdb() -> TestPg {
        dotenvy::from_filename(".env.test").ok();
        let server_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
        let migrations = Path::new("../migrations");
        TestPg::new(server_url, migrations)
    }
}
