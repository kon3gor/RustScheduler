use chrono::{Weekday, DateTime, FixedOffset, Datelike, NaiveTime, Duration};
use std::str::FromStr;

pub fn check_day(day: &String, now: &DateTime<FixedOffset>) -> bool {
    return match day.parse::<Weekday>() {
        Ok(v) => v == now.weekday(),
        Err(_) => day.as_str() == "every",
    };
}

pub fn check_time(time: &str, now: &DateTime<FixedOffset>) -> bool {
    let naive_time = match NaiveTime::from_str(time) {
        Ok(v) => v,
        Err(e) => {
            eprintln!("Error wile parsing time string: {}", e);
            return false;
        }
    };
    let duration = now.time().signed_duration_since(naive_time);
    return duration < Duration::seconds(1) && duration > Duration::seconds(0);
}
