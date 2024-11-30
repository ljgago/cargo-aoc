use chrono::{DateTime, Utc};

pub fn last_year() -> u32 {
    // Fix the timestamp with the EST (UTC-5) timezone
    let timestamp = Utc::now().timestamp() - 5 * 60 * 60;
    let now = DateTime::from_timestamp(timestamp, 0).unwrap();
    let year = format!("{}", now.format("%Y")).parse::<u32>().unwrap();
    let month = format!("{}", now.format("%m")).parse::<u32>().unwrap();

    if month == 12 {
        year
    } else {
        year - 1
    }
}
