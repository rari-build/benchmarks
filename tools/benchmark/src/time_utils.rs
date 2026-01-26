use std::time::{SystemTime, UNIX_EPOCH};

pub fn format_timestamp(time: SystemTime) -> String {
    let duration = time.duration_since(UNIX_EPOCH).unwrap();
    let secs = duration.as_secs();

    let days_since_epoch = secs / 86400;
    let mut year = 1970;
    let mut days = days_since_epoch;

    loop {
        let days_in_year = if year % 4 == 0 && (year % 100 != 0 || year % 400 == 0) {
            366
        } else {
            365
        };
        if days < days_in_year {
            break;
        }
        days -= days_in_year;
        year += 1;
    }

    let days_in_months = if year % 4 == 0 && (year % 100 != 0 || year % 400 == 0) {
        [31, 29, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31]
    } else {
        [31, 28, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31]
    };

    let mut month = 1;
    for &days_in_month in &days_in_months {
        if days < days_in_month {
            break;
        }
        days -= days_in_month;
        month += 1;
    }
    let day = days + 1;

    let time_of_day = secs % 86400;
    let hour = time_of_day / 3600;
    let minute = (time_of_day % 3600) / 60;
    let second = time_of_day % 60;

    format!(
        "{:04}-{:02}-{:02}T{:02}:{:02}:{:02}Z",
        year, month, day, hour, minute, second
    )
}

pub fn format_date(time: SystemTime) -> String {
    format_timestamp(time)
        .split('T')
        .next()
        .unwrap()
        .to_string()
}

#[allow(dead_code)]
pub fn serialize_float_as_int_if_whole<S>(value: &f64, serializer: S) -> Result<S::Ok, S::Error>
where
    S: serde::Serializer,
{
    if value.fract() == 0.0 {
        serializer.serialize_i64(*value as i64)
    } else {
        serializer.serialize_f64(*value)
    }
}
