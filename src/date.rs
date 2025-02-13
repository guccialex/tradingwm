use chrono::{NaiveDate, TimeZone, Utc};

pub fn convert_date_to_epoch(date_str: &str) -> Result<i64, chrono::ParseError> {

    let naive_date = NaiveDate::parse_from_str(date_str.trim(), "%B %d, %Y")?;
    let datetime = Utc.from_utc_date(&naive_date).and_hms(0, 0, 0);
    Ok(datetime.timestamp())
}


/// A simple wrapper around an epoch time.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Date(pub i64);

impl Date {
    /// Creates a `Date` from a string representation.
    /// Panics if the string cannot be parsed.
    pub fn from_string(s: &str) -> Self {
        Date(convert_date_to_epoch(s).unwrap())
    }
}