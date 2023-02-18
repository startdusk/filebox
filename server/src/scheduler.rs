use chrono::Utc;
use sqlx::PgPool;
use std::fs;
use tokio_schedule::{every, Job};

use crate::{dbaccess::filebox::delete_expired_filebox_db, models::filebox::FileType};

pub async fn start_clean_expired_filebox(pool: &PgPool, upload_path: String) {
    every(1)
        .hours()
        .in_timezone(&Utc)
        .perform(|| async {
            log::info!("start_clean_expired_filebox event - start");
            match delete_expired_filebox_db(pool).await {
                Ok(filebox_vec) => {
                    // clean expired path
                    for filebox in &filebox_vec {
                        if filebox.file_type == FileType::File {
                            let file_path = format!("{}/{}", upload_path, filebox.file_path);
                            let _ = fs::remove_file(file_path);
                        }
                    }
                }
                Err(err) => log::error!("start_clean_expired_filebox event - failed {:?}", err),
            }
            log::info!("start_clean_expired_filebox event - end");
        })
        .await;
}
