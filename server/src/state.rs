use chrono::{Duration, Local};
use sqlx::postgres::PgPool;
use std::cell::RefCell;
use tiny_id::ShortCodeGenerator;

use crate::api::IpInfo;

#[derive(Debug)]
pub struct AppState {
    pub health_check_response: String,
    pub visit_count: std::sync::Mutex<u64>,
    pub upload_path: String,
    pub db: PgPool,

    // 由于会 标准库中的 Mutex 在 .await中 会: this `MutexGuard` is held across an `await` point
    // 所以改用 tokio 的 Mutex
    pub code_gen: tokio::sync::Mutex<RefCell<ShortCodeGenerator<char>>>,
}

pub struct FileboxState {
    pub ip_allower: tokio::sync::Mutex<RefCell<IPAllower>>,
}

pub struct IPAllower {
    limit: i32,
    duration_day: i64,
    con: redis::Connection,
}

impl IPAllower {
    pub fn new(con: redis::Connection, limit: i32, duration_day: i64) -> Self {
        Self {
            limit,
            duration_day,
            con,
        }
    }

    pub fn allow_ip(&mut self, ip: String) -> bool {
        let (ip_info, get_it) = self.get(ip);
        if !get_it {
            return true;
        }

        if ip_info.count >= self.limit {
            return false;
        }
        true
    }

    pub fn add_ip(&mut self, ip: String) {
        let (mut ip_info, get_it) = self.get(ip.clone());
        if get_it {
            ip_info.count += 1;
        }
        dbg!(&ip_info);

        let value = serde_json::to_string(&ip_info).unwrap();
        let _: bool = redis::cmd("SETEX")
            .arg(ip)
            .arg(self.duration())
            .arg(value)
            .query(&mut self.con)
            .unwrap();
    }

    fn get(&mut self, ip: String) -> (IpInfo, bool) {
        // TODO: don't unwrap
        match redis::cmd("GET").arg(ip).query(&mut self.con) {
            Ok(ip_info_json) => {
                let ip_info_json: String = ip_info_json;
                let ip_info: IpInfo = ip_info_json.try_into().unwrap();
                (ip_info, true)
            }
            Err(_) => (IpInfo::default(), false),
        }
    }

    fn duration(&self) -> i64 {
        let now = Local::now();

        let tomorrow_midnight = (now + Duration::days(self.duration_day))
            .date_naive()
            .and_hms_opt(0, 0, 0)
            .unwrap();

        tomorrow_midnight
            .signed_duration_since(now.naive_local())
            .to_std()
            .unwrap()
            .as_secs() as i64
    }
}

impl TryFrom<String> for IpInfo {
    type Error = serde_json::Error;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        serde_json::from_str(&value)
    }
}
