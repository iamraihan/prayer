//! Pure schedule computation: turns a day's timings + settings into the list
//! of alarm events for that day. No I/O, easy to unit-test and extend.

use chrono::{DateTime, Duration, NaiveTime};
use chrono_tz::Tz;

use crate::aladhan::DayTimings;
use crate::models::{Prayer, Settings};
use crate::timeutil;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum EventKind {
    /// The prayer's main alarm.
    Start,
    /// Final warning before the window closes.
    PreEnd,
}

impl EventKind {
    pub fn id(self) -> &'static str {
        match self {
            EventKind::Start => "start",
            EventKind::PreEnd => "preEnd",
        }
    }
}

#[derive(Debug, Clone)]
pub struct PlannedEvent {
    pub kind: EventKind,
    /// True when the main alarm is delayed past the start time
    /// (reminder_minutes set) — the engine uses the "you still have …"
    /// message for it.
    pub delayed: bool,
    pub prayer: Prayer,
    /// When this event fires.
    pub at: DateTime<Tz>,
    /// Window start (the prayer's own time).
    pub start: DateTime<Tz>,
    /// Window end (when the prayer's time closes).
    pub end: DateTime<Tz>,
}

/// The end of a prayer's valid window:
/// Fajr → sunrise, Dhuhr → Asr, Asr → Maghrib, Maghrib → Isha, Isha → midnight.
pub fn end_time_of(prayer: Prayer, day: &DayTimings) -> NaiveTime {
    let fallback = |a: Option<&NaiveTime>, b: NaiveTime| a.copied().unwrap_or(b);
    match prayer {
        Prayer::Fajr => day.sunrise,
        Prayer::Dhuhr => fallback(
            day.times.get(&Prayer::Asr),
            fallback(day.times.get(&Prayer::Maghrib), day.sunrise),
        ),
        Prayer::Asr => fallback(day.times.get(&Prayer::Maghrib), day.sunrise),
        Prayer::Maghrib => fallback(day.times.get(&Prayer::Isha), day.sunrise),
        Prayer::Isha => day.midnight,
    }
}

pub fn compute(day: &DayTimings, tz: Tz, settings: &Settings) -> Vec<PlannedEvent> {
    let mut events = Vec::new();

    for prayer in Prayer::ALL {
        let Some(&start_t) = day.times.get(&prayer) else {
            continue;
        };
        let ps = settings.prayer_settings.get(&prayer);
        let enabled = ps.map(|p| p.enabled).unwrap_or(true);
        if !enabled {
            continue;
        }

        let end_t = end_time_of(prayer, day);
        let Some(start) = timeutil::resolve(tz, day.date, start_t) else {
            continue;
        };
        let Some(end) = timeutil::resolve_after(tz, day.date, end_t, start_t) else {
            continue;
        };
        if end <= start {
            continue;
        }

        // Main alarm: exactly at the start time, or delayed N minutes after
        // it when a value is set ("you still have … minutes"). The delay is
        // clamped so the alarm never lands in the last 5 minutes of the
        // window (that's the before-end warning's job).
        let delay = ps.and_then(|p| p.reminder_minutes);
        let (at, delayed) = match delay {
            Some(mins) => {
                let target = start
                    .checked_add_signed(Duration::minutes(mins as i64))
                    .unwrap_or(start);
                let latest = end
                    .checked_sub_signed(Duration::minutes(5))
                    .unwrap_or(start);
                let at = if latest > start {
                    target.min(latest)
                } else {
                    start
                };
                (at, at > start)
            }
            None => (start, false),
        };
        events.push(PlannedEvent {
            kind: EventKind::Start,
            delayed,
            prayer,
            at,
            start,
            end,
        });

        // Final warning shortly before the window closes ("you will miss it").
        if settings.pre_end_warning_enabled && settings.pre_end_warning_minutes > 0 {
            if let Some(at) =
                end.checked_sub_signed(Duration::minutes(settings.pre_end_warning_minutes as i64))
            {
                if at > start + Duration::minutes(2) {
                    events.push(PlannedEvent {
                        kind: EventKind::PreEnd,
                        delayed: false,
                        prayer,
                        at,
                        start,
                        end,
                    });
                }
            }
        }
    }

    events
}

/// Unique id for an event within its day, used to track "already fired".
pub fn event_id(date: chrono::NaiveDate, prayer: Prayer, kind: EventKind) -> String {
    format!("{date}|{}|{}", prayer.id(), kind.id())
}
