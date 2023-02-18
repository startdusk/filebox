use chrono::{Duration, Local};

use crate::api::IpInfo;
use redis::Connection;

pub struct IpAllower {
    pub conn: Connection,
    pub limit: i32,
    pub duration_day: i64,
}

impl IpAllower {
    pub fn new(conn: Connection, limit: i32, duration_day: i64) -> Self {
        IpAllower {
            conn,
            limit,
            duration_day,
        }
    }
}

pub fn allow_ip(conn: &mut Connection, ip: String, limit: i32) -> bool {
    let (ip_info, get_it) = get(conn, ip);
    if !get_it {
        return true;
    }

    if ip_info.count >= limit {
        return false;
    }
    true
}

pub fn add_ip(conn: &mut Connection, ip: String, duration_day: i64) -> bool {
    let (mut ip_info, get_it) = get(conn, ip.clone());
    if get_it {
        ip_info.count += 1;
    }

    let value = serde_json::to_string(&ip_info).unwrap();
    redis::cmd("SETEX")
        .arg(ip)
        .arg(duration(duration_day))
        .arg(value)
        .query(conn)
        .unwrap()
}

fn get(conn: &mut Connection, ip: String) -> (IpInfo, bool) {
    match redis::cmd("GET").arg(ip).query(conn) {
        Ok(ip_info_json) => {
            let ip_info_json: String = ip_info_json;
            let ip_info: IpInfo = ip_info_json.try_into().unwrap();
            (ip_info, true)
        }
        Err(_) => (IpInfo::default(), false),
    }
}

fn duration(duration_day: i64) -> i64 {
    let now = Local::now();

    let tomorrow_midnight = (now + Duration::days(duration_day))
        .date_naive()
        .and_hms_opt(0, 0, 0)
        .unwrap();

    tomorrow_midnight
        .signed_duration_since(now.naive_local())
        .to_std()
        .unwrap()
        .as_secs() as i64
}
