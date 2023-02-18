use api::IpInfo;
use chrono::{Duration, Local};

pub mod api;
pub mod dbaccess;
pub mod errors;
pub mod handlers;
pub mod middlewares;
pub mod models;
pub mod scheduler;
pub mod state;

#[cfg(test)]
pub mod test_utils;

#[cfg(test)]
mod tests;

pub struct IPAllower {
    pub limit: i32,
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

    pub fn add_ip(&mut self, ip: String) -> bool {
        let (mut ip_info, get_it) = self.get(ip.clone());
        if get_it {
            ip_info.count += 1;
        }

        let value = serde_json::to_string(&ip_info).unwrap();
        redis::cmd("SETEX")
            .arg(ip)
            .arg(self.duration())
            .arg(value)
            .query(&mut self.con)
            .unwrap()
    }

    fn get(&mut self, ip: String) -> (IpInfo, bool) {
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
