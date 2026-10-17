//! Per-day scheduler runtime state (fired alarms, snoozes, done marks),
//! persisted so a restart doesn't replay already-fired alarms.

use std::collections::HashMap;
use std::path::Path;

use chrono::{DateTime, NaiveDate, Utc};
use serde::{Deserialize, Serialize};

use crate::models::Prayer;

#[derive(Debug, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", default)]
pub struct RuntimeState {
    /// Location-local day this state belongs to; reset on rollover.
    pub day: Option<NaiveDate>,
    /// Fired event ids ("2026-10-17|dhuhr|start").
    pub fired: Vec<String>,
    /// prayer -> due instant (RFC3339 UTC)
    pub snoozes: HashMap<Prayer, String>,
    /// prayer -> done instant (RFC3339 UTC)
    pub done: HashMap<Prayer, String>,
}

pub fn load(dir: &Path) -> RuntimeState {
    std::fs::read_to_string(dir.join("runtime.json"))
        .ok()
        .and_then(|text| serde_json::from_str(&text).ok())
        .unwrap_or_default()
}

pub fn save(dir: &Path, state: &RuntimeState) {
    if let Ok(json) = serde_json::to_string(state) {
        let _ = std::fs::write(dir.join("runtime.json"), json);
    }
}

pub fn encode_dt(dt: &DateTime<Utc>) -> String {
    dt.to_rfc3339()
}

pub fn decode_dt(s: &str) -> Option<DateTime<Utc>> {
    DateTime::parse_from_rfc3339(s)
        .ok()
        .map(|dt| dt.with_timezone(&Utc))
}
