//! Timezone-aware helpers. All scheduling math happens in the *location's*
//! IANA timezone (from the Aladhan API), not the device timezone.

use chrono::{DateTime, LocalResult, NaiveDate, NaiveTime};
use chrono_tz::Tz;

pub fn parse_tz(name: &str) -> Option<Tz> {
    name.parse().ok()
}

/// Combines a date + wall-clock time into an instant in `tz`, tolerating
/// DST gaps/overlaps (first candidate wins on ambiguity).
pub fn resolve(tz: Tz, date: NaiveDate, time: NaiveTime) -> Option<DateTime<Tz>> {
    match date.and_time(time).and_local_timezone(tz) {
        LocalResult::Single(dt) => Some(dt),
        LocalResult::Ambiguous(dt, _) => Some(dt),
        LocalResult::None => None,
    }
}

/// Like [`resolve`] but rolls to the next day when `time` is earlier than
/// `earlier_than` (e.g. Isha ending at Midnight "00:30" of the next day).
pub fn resolve_after(
    tz: Tz,
    date: NaiveDate,
    time: NaiveTime,
    earlier_than: NaiveTime,
) -> Option<DateTime<Tz>> {
    if time >= earlier_than {
        resolve(tz, date, time)
    } else {
        let next_day = date.succ_opt()?;
        resolve(tz, next_day, time)
    }
}
