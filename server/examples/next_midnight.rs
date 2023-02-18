use chrono::{Duration, Local};

fn main() {
    let now = Local::now();

    let tomorrow_midnight = (now + Duration::days(1))
        .date_naive()
        .and_hms_opt(0, 0, 0)
        .unwrap();

    let duration = tomorrow_midnight
        .signed_duration_since(now.naive_local())
        .to_std()
        .unwrap();

    println!(
        "Duration between {:?} and {:?}: {:?}",
        now, tomorrow_midnight, duration
    );
}
