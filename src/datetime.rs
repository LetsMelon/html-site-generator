use chrono::NaiveDate;

#[derive(Debug)]
pub struct DateTime {
    inner: NaiveDate,
}

impl DateTime {
    pub fn from_ymd_opt(year: i32, month: u32, day: u32) -> Option<DateTime> {
        Some(DateTime {
            inner: NaiveDate::from_ymd_opt(year, month, day)?,
        })
    }
}
