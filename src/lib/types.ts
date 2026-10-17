/**
 * Shared types mirroring the Rust DTOs in `src-tauri/src/models.rs`.
 * Keep both sides in sync when changing a shape.
 */

export type Prayer = "fajr" | "dhuhr" | "asr" | "maghrib" | "isha";

export const PRAYERS: Prayer[] = ["fajr", "dhuhr", "asr", "maghrib", "isha"];

export const PRAYER_LABELS: Record<Prayer, string> = {
  fajr: "Fajr",
  dhuhr: "Dhuhr",
  asr: "Asr",
  maghrib: "Maghrib",
  isha: "Isha",
};

export const PRAYER_EMOJIS: Record<Prayer, string> = {
  fajr: "🌅",
  dhuhr: "☀️",
  asr: "🌤️",
  maghrib: "🌇",
  isha: "🌙",
};

export interface Location {
  city: string;
  country: string;
  countryCode: string;
  timezone: string;
}

export interface PrayerReminderSettings {
  /** Master switch for this prayer's alarms (start / reminder / pre-end). */
  enabled: boolean;
  /** Minutes after start for the "you still have N minutes" reminder. `null` disables it (default for Maghrib). */
  reminderMinutes: number | null;
}

export interface Settings {
  location: Location | null;
  method: number;
  /** 0 = Standard (Shafi), 1 = Hanafi — affects Asr. */
  school: 0 | 1;
  timeFormat: "12h" | "24h";
  notificationsEnabled: boolean;
  soundEnabled: boolean;
  /** 0.0 – 1.0 */
  soundVolume: number;
  popupEnabled: boolean;
  preEndWarningEnabled: boolean;
  preEndWarningMinutes: number;
  prayerSettings: Record<Prayer, PrayerReminderSettings>;
}

export interface PrayerTime {
  prayer: Prayer;
  /** "HH:MM" in the location's timezone. */
  start: string;
  /** "HH:MM" in the location's timezone. */
  ends: string;
}

export interface DayTimings {
  /** Location-local date, "YYYY-MM-DD". */
  date: string;
  gregorian: string;
  hijri: string;
  sunrise: string;
  midnight: string;
  timings: PrayerTime[];
  /** True when serving from the offline disk cache instead of a fresh API response. */
  fromCache: boolean;
}

export type AlarmKind = "start" | "reminder" | "preEnd" | "snooze";

export interface AlarmMessage {
  title: string;
  body: string;
}

export interface ActiveAlarm {
  id: string;
  prayer: Prayer;
  kind: AlarmKind;
  startedAt: string;
  message: AlarmMessage;
}

export interface NextPrayerInfo {
  prayer: Prayer;
  startAt: string;
  endsAt: string;
}

export interface AppState {
  hasLocation: boolean;
  settings: Settings;
  today: DayTimings | null;
  next: NextPrayerInfo | null;
  doneToday: Partial<Record<Prayer, true>>;
  activeAlarm: ActiveAlarm | null;
  apiError: string | null;
}

export interface LocationPreview {
  location: Location;
  day: DayTimings;
}
