use chrono::prelude::*;

pub fn now_timestamp() -> i64 {
    let utc: DateTime<Utc> = Utc::now();
    utc.timestamp()
}

pub fn now() -> DateTime<Utc> {
    Utc::now()
}

pub fn unix_to_time(unix: i64) -> DateTime<Utc> {
    DateTime::from_timestamp(unix, 0).unwrap()
}
