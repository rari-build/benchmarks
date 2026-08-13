use std::time::{SystemTime, UNIX_EPOCH};

fn is_leap_year(year: u64) -> bool {
    year.is_multiple_of(4) && (!year.is_multiple_of(100) || year.is_multiple_of(400))
}

#[must_use]
pub fn format_timestamp(time: SystemTime) -> String {
    let duration = time.duration_since(UNIX_EPOCH).unwrap_or_default();
    let secs = duration.as_secs();

    let days_since_epoch = secs / 86400;
    let mut year = 1970_u64;
    let mut days = days_since_epoch;

    loop {
        let days_in_year = if is_leap_year(year) { 366 } else { 365 };
        if days < days_in_year {
            break;
        }
        days -= days_in_year;
        year += 1;
    }

    let days_in_months = if is_leap_year(year) {
        [31_u64, 29, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31]
    } else {
        [31, 28, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31]
    };

    let mut month = 1_u64;
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

    format!("{year:04}-{month:02}-{day:02}T{hour:02}:{minute:02}:{second:02}Z")
}

#[must_use]
pub fn format_date(time: SystemTime) -> String {
    let timestamp = format_timestamp(time);
    match timestamp.split_once('T') {
        Some((date, _)) => date.to_string(),
        None => timestamp,
    }
}

/// Serializes a float as an integer when it has no fractional part.
///
/// # Errors
///
/// Returns any error from the underlying [`serde::Serializer`].
pub fn serialize_float_as_int_if_whole<S>(value: &f64, serializer: S) -> Result<S::Ok, S::Error>
where
    S: serde::Serializer,
{
    if value.fract() == 0.0 {
        #[expect(clippy::cast_possible_truncation)]
        let as_int = *value as i64;
        serializer.serialize_i64(as_int)
    } else {
        serializer.serialize_f64(*value)
    }
}

#[must_use]
pub fn round_ms(value: f64) -> f64 {
    (value * 1000.0).round() / 1000.0
}

#[must_use]
pub fn round_2(value: f64) -> f64 {
    (value * 100.0).round() / 100.0
}
