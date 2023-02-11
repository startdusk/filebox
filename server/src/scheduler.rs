use chrono::{Local, Utc};
use sqlx::PgPool;
use tokio_schedule::{every, Job};

use crate::dbaccess::filebox::delete_expired_filebox_db;

pub async fn start_clean_expired_filebox(pool: &PgPool) {
    let every_second = every(1).hours().in_timezone(&Utc).perform(|| async {
        println!(
            "start_clean_expired_filebox event - {:?} start",
            Local::now()
        );
        if let Err(err) = delete_expired_filebox_db(pool).await {
            println!(
                "start_clean_expired_filebox event - {:?}: failed {:?}",
                Local::now(),
                err
            );
        }

        println!("start_clean_expired_filebox event - {:?} end", Local::now());
    });
    every_second.await;
}
