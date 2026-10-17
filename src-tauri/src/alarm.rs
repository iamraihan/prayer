//! Alarm delivery: system notification, the always-on-top popup window and
//! human-readable message copy.

use chrono::{DateTime, Timelike};
use chrono_tz::Tz;
use tauri::{AppHandle, Manager, WebviewUrl, webview::WebviewWindowBuilder};
use tauri_plugin_notification::NotificationExt;

use crate::models::AlarmMessageDto;

pub const ALARM_WINDOW: &str = "alarm";

/// Creates the popup window once, hidden; it is only shown while an alarm is
/// active. Created eagerly so the first alarm shows instantly.
pub fn ensure_window(app: &AppHandle) {
    if app.get_webview_window(ALARM_WINDOW).is_some() {
        return;
    }
    let _ = WebviewWindowBuilder::new(
        app,
        ALARM_WINDOW,
        WebviewUrl::App("alarm.html".into()),
    )
    .title("Prayer Alarm")
    .inner_size(460.0, 580.0)
    .resizable(false)
    .decorations(false)
    .skip_taskbar(true)
    .always_on_top(true)
    .visible(false)
    .center()
    .build();
}

pub fn show_window(app: &AppHandle) {
    if let Some(win) = app.get_webview_window(ALARM_WINDOW) {
        let _ = win.center();
        let _ = win.show();
        let _ = win.set_focus();
    }
}

pub fn hide_window(app: &AppHandle) {
    if let Some(win) = app.get_webview_window(ALARM_WINDOW) {
        let _ = win.hide();
    }
}

pub fn notify(app: &AppHandle, message: &AlarmMessageDto) {
    let _ = app
        .notification()
        .builder()
        .title(&message.title)
        .body(&message.body)
        .show();
}

/// "5:45 PM" or "17:45" in the location's timezone.
pub fn fmt_time(t: &DateTime<Tz>, mode_24h: bool) -> String {
    let (h, m) = (t.hour(), t.minute());
    if mode_24h {
        format!("{h:02}:{m:02}")
    } else {
        let suffix = if h < 12 { "AM" } else { "PM" };
        let h12 = match h % 12 {
            0 => 12,
            other => other,
        };
        format!("{h12}:{m:02} {suffix}")
    }
}
