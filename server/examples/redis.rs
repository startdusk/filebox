use std::{thread, time};

use server::api::IpInfo;

fn main() {
    let client = redis::Client::open("redis://127.0.0.1").unwrap();
    let mut con = client.get_connection().unwrap();

    let ip = "127.0.0.1";
    let mut ip_info = IpInfo::new();
    ip_info.visit_error_limit_of_per_day += 1;
    let value = serde_json::to_string(&ip_info).unwrap();
    let res: bool = redis::cmd("SETEX")
        .arg(ip)
        .arg(3)
        .arg(value)
        .query(&mut con)
        .unwrap();
    dbg!(res);
    let ip_info_json: String = redis::cmd("GET").arg(ip).query(&mut con).unwrap();
    dbg!(&ip_info_json);
    let ip_info: IpInfo = ip_info_json.try_into().unwrap();
    dbg!(ip_info);

    thread::sleep(time::Duration::from_secs(3));

    let res: bool = redis::cmd("GET").arg(ip).query(&mut con).unwrap();
    dbg!(res); // false
}
