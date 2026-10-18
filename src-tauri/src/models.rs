//! Domain models and DTOs shared with the frontend (camelCase on the wire).
//!
//! The TypeScript twins live in `src/lib/types.ts` — keep them in sync.

use std::collections::HashMap;
use serde::{Deserialize, Serialize};

// ---------------------------------------------------------------------------
// Core domain
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Prayer {
    Fajr,
    Dhuhr,
    Asr,
    Maghrib,
    Isha,
}

impl Prayer {
    pub const ALL: [Prayer; 5] = [
        Prayer::Fajr,
        Prayer::Dhuhr,
        Prayer::Asr,
        Prayer::Maghrib,
        Prayer::Isha,
    ];

    pub fn id(self) -> &'static str {
        match self {
            Prayer::Fajr => "fajr",
            Prayer::Dhuhr => "dhuhr",
            Prayer::Asr => "asr",
            Prayer::Maghrib => "maghrib",
            Prayer::Isha => "isha",
        }
    }

    pub fn title(self) -> &'static str {
        match self {
            Prayer::Fajr => "Fajr",
            Prayer::Dhuhr => "Dhuhr",
            Prayer::Asr => "Asr",
            Prayer::Maghrib => "Maghrib",
            Prayer::Isha => "Isha",
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Location {
    pub city: String,
    /// Human-readable country name (display only).
    pub country: String,
    /// ISO 3166-1 alpha-2 code — what the Aladhan API expects.
    pub country_code: String,
    /// IANA timezone of the location (times are interpreted in this zone).
    pub timezone: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PrayerSettings {
    pub enabled: bool,
    /// Minutes after the prayer starts for the mid-window reminder.
    /// `None` disables the reminder (default for Maghrib).
    pub reminder_minutes: Option<u32>,
}

impl PrayerSettings {
    pub fn default_for(prayer: Prayer) -> Self {
        Self {
            enabled: true,
            // Maghrib's window is short: no mid-window reminder by default.
            reminder_minutes: if prayer == Prayer::Maghrib {
                None
            } else {
                Some(30)
            },
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Settings {
    pub location: Option<Location>,
    /// Aladhan calculation method id.
    pub method: u8,
    /// 0 = Standard (Shafi), 1 = Hanafi — affects Asr.
    pub school: u8,
    /// "12h" | "24h"
    pub time_format: String,
    pub notifications_enabled: bool,
    pub sound_enabled: bool,
    pub sound_volume: f32,
    /// File name of a custom alarm tone inside the app config dir
    /// (e.g. "alarm-tone.mp3"); `None` = built-in chime.
    pub alarm_sound: Option<String>,
    pub popup_enabled: bool,
    pub pre_end_warning_enabled: bool,
    pub pre_end_warning_minutes: u32,
    pub prayer_settings: HashMap<Prayer, PrayerSettings>,
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            location: None,
            method: 3, // Muslim World League
            school: 0,
            time_format: "12h".to_string(),
            notifications_enabled: true,
            sound_enabled: true,
            sound_volume: 0.8,
            alarm_sound: None,
            popup_enabled: true,
            pre_end_warning_enabled: true,
            pre_end_warning_minutes: 10,
            prayer_settings: Prayer::ALL
                .iter()
                .map(|p| (*p, PrayerSettings::default_for(*p)))
                .collect(),
        }
    }
}

impl Settings {
    /// Fills in any missing per-prayer settings (forward compatibility when
    /// settings.json comes from an older version).
    pub fn normalized(mut self) -> Self {
        for p in Prayer::ALL {
            self.prayer_settings
                .entry(p)
                .or_insert_with(|| PrayerSettings::default_for(p));
        }
        self
    }
}

// ---------------------------------------------------------------------------
// DTOs pushed to the frontend
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PrayerTimeDto {
    pub prayer: Prayer,
    /// "HH:MM" location-local
    pub start: String,
    /// "HH:MM" location-local
    pub ends: String,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DayTimingsDto {
    /// "YYYY-MM-DD" location-local
    pub date: String,
    pub gregorian: String,
    pub hijri: String,
    pub sunrise: String,
    pub midnight: String,
    pub timings: Vec<PrayerTimeDto>,
    pub from_cache: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AlarmMessageDto {
    pub title: String,
    pub body: String,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ActiveAlarmDto {
    pub id: String,
    pub prayer: Prayer,
    /// "start" | "reminder" | "preEnd" | "snooze"
    pub kind: String,
    pub started_at: String,
    pub message: AlarmMessageDto,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct NextPrayerDto {
    pub prayer: Prayer,
    pub start_at: String,
    pub ends_at: String,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AppStateDto {
    pub has_location: bool,
    pub settings: Settings,
    pub today: Option<DayTimingsDto>,
    pub next: Option<NextPrayerDto>,
    pub done_today: HashMap<Prayer, bool>,
    pub active_alarm: Option<ActiveAlarmDto>,
    pub api_error: Option<String>,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct LocationPreviewDto {
    pub location: Location,
    pub day: DayTimingsDto,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PreviewRequest {
    pub city: String,
    pub country: String,
    pub country_code: String,
    pub method: u8,
    pub school: u8,
}
