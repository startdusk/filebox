use actix::Addr;
use actix_redis::{resp_array, Command, RedisActor, RespValue};
use chrono::{Duration, Local};

use crate::{api::IpInfo, errors};

pub struct IpAllower {
    pub limit: i32,
    pub ttl: i64,
}

impl IpAllower {
    pub fn new(limit: i32, ttl: i64) -> Self {
        Self { limit, ttl }
    }
}

pub async fn allow_ip(
    addr: &Addr<RedisActor>,
    ip: &str,
    limit: i32,
) -> Result<bool, errors::Error> {
    let (ip_info, get_it) = get(addr, ip).await?;
    if !get_it {
        return Ok(true);
    }

    if ip_info.count >= limit {
        return Ok(false);
    }
    Ok(true)
}

pub async fn add_ip(addr: &Addr<RedisActor>, ip: &str, ttl: i64) -> Result<(), errors::Error> {
    let (mut ip_info, get_it) = get(addr, ip).await?;
    if get_it {
        ip_info.count += 1;
    }

    // SAFTEY: we ensure the ip_info implementation of Serialize
    let value = serde_json::to_string(&ip_info).unwrap();
    let ttl = get_ttl(ttl);
    let cmd = Command(resp_array!["SETEX", ip, ttl.to_string(), value]);
    if let RespValue::Error(msg) = addr
        .send(cmd)
        .await
        .map_err(Into::into)
        .map_err(errors::Error::RedisError)?
        .map_err(Into::into)
        .map_err(errors::Error::RedisError)?
    {
        return Err(errors::Error::RedisSendCommandError(msg));
    };

    Ok(())
}

async fn get(addr: &Addr<RedisActor>, ip: &str) -> Result<(IpInfo, bool), errors::Error> {
    let cmd = Command(resp_array!["GET", ip]);
    let val = addr
        .send(cmd)
        .await
        .map_err(Into::into)
        .map_err(errors::Error::RedisError)?
        .map_err(Into::into)
        .map_err(errors::Error::RedisError)?;
    match val {
        RespValue::BulkString(ip_info_vec) => {
            let ip_info_json: String = String::from_utf8(ip_info_vec)?;
            let ip_info: IpInfo = ip_info_json
                .try_into()
                .map_err(errors::Error::DeserializeJsonError)?;

            Ok((ip_info, true))
        }
        _ => Ok((IpInfo::default(), false)),
    }
}

fn get_ttl(ttl: i64) -> i64 {
    let now = Local::now();

    let tomorrow_midnight = (now + Duration::days(ttl))
        .date_naive()
        .and_hms_opt(0, 0, 0)
        .unwrap();

    tomorrow_midnight
        .signed_duration_since(now.naive_local())
        .to_std()
        .unwrap()
        .as_secs() as i64
}
