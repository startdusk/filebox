use chrono::Utc;
use sqlx::PgPool;
use tokio_schedule::{every, Job};

use crate::dbaccess::filebox::delete_expired_filebox_db;

pub async fn start_clean_expired_filebox(pool: &PgPool) {
    let every_second = every(1).hours().in_timezone(&Utc).perform(|| async {
        log::info!("start_clean_expired_filebox event - start");
        if let Err(err) = delete_expired_filebox_db(pool).await {
            log::debug!("start_clean_expired_filebox event - failed {:?}", err);
        }
        log::info!("start_clean_expired_filebox event - end");
    });
    every_second.await;
}
