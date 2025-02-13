use chrono::{NaiveDate, TimeZone, Utc};

/// Converts a date string in the format "December 23, 2024" into an epoch time (seconds since Unix epoch).
///
/// # Arguments
///
/// * `date_str` - A string slice containing the date in the format "%B %d, %Y".
///
/// # Returns
///
/// * On success, returns the epoch time as an `i64`.
/// * On failure, returns a `chrono::ParseError`.
pub fn convert_date_to_epoch(date_str: &str) -> Result<i64, chrono::ParseError> {
    let naive_date = NaiveDate::parse_from_str(date_str.trim(), "%B %d, %Y")?;
    let datetime = Utc.from_utc_date(&naive_date).and_hms(0, 0, 0);
    Ok(datetime.timestamp())
}