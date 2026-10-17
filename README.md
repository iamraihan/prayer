# Prayer Times

A cross-platform (Windows, macOS, Ubuntu/Linux) prayer-times app built with
Tauri 2, using the [Aladhan API](https://aladhan.com/prayer-times-api).

- **System tray status** — next prayer, countdown and window end time in the
  tray tooltip/menu (Windows taskbar corner, macOS menu bar, Ubuntu app
  indicator). Left-click the tray icon to open the app; closing the window
  keeps the app running in the tray.
- **Alarms that follow the whole prayer window** — see the table below.
- **First-run wizard** — pick country + city, calculation method and Asr
  school; a live preview is fetched before anything is saved.
- **Offline-friendly** — each month's timetable is cached on disk
  (one API call per month in the happy path).

## Alarm behavior

For each prayer (Fajr, Dhuhr, Asr, Maghrib, Isha) the scheduler plans:

| Alarm        | When it fires                                              | Default |
| ------------ | ---------------------------------------------------------- | ------- |
| Start        | exactly at the prayer's start time                         | on      |
| Reminder     | N minutes after start — "you still have N minutes"          | +30 min (off for Maghrib — short window) |
| Before-end   | N minutes before the window closes — "you will miss it"     | 10 min  |
| Snooze       | when a "Remind me again in N minutes" answer is due         | user-set minutes |

Each alarm shows **all three channels** (each can be toggled in Settings):

1. a system notification (Windows/macOS notification center, Ubuntu daemon),
2. a looping chime sound (auto-stops after 2 minutes if unanswered),
3. a small always-on-top popup with the actions below.

From the popup, the in-app banner, or the tray menu you can:

- **✓ Mark as done** — silences *all remaining alarms for that prayer today*
  (e.g. marking Dhuhr done stops its reminder and before-end warning).
- **Remind me again in N minutes** — preset chips (5/10/15/30) or a custom
  number of minutes; the alarm re-fires then.
- **Dismiss for now** — stops the sound; later reminders still fire because
  the prayer isn't marked done.

A prayer window closes at: Fajr → sunrise, Dhuhr → Asr, Asr → Maghrib,
Maghrib → Isha, Isha → midnight.

## Tech stack

- **Frontend**: React 19 + TypeScript + Vite + Tailwind CSS v4, zustand store,
  lucide icons. Two pages builds: `index.html` (main window) and
  `alarm.html` (popup window).
- **Backend**: Rust (Tauri 2) — scheduling, tray, alarms, caching.
  `chrono-tz` for location-timezone-correct math, `rodio` for a synthesized
  chime (no audio assets), `reqwest` (rustls) for the Aladhan API.

See **ARCHITECTURE.md** for the module map and how to extend the app.

## Getting started

Prerequisites: [Node](https://nodejs.org) 20+, [pnpm](https://pnpm.io), and a
Rust toolchain per the [Tauri prerequisites](https://tauri.app/start/prerequisites/).

### One-time setup on Windows

```powershell
winget install Rustlang.Rustup
winget install Microsoft.VisualStudio.2022.BuildTools --override "--wait --passive --add Microsoft.VisualStudio.Workload.VCTools --includeRecommended"
```

(WebView2 is pre-installed on Windows 10/11.)

### One-time setup on Ubuntu

```bash
sudo apt update
sudo apt install -y build-essential curl libssl-dev libgtk-3-dev libayatana-appindicator3-dev librsvg2-dev libwebkit2gtk-4.1-dev libasound2-dev
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

### One-time setup on macOS

```bash
xcode-select --install
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

### Run / build

```bash
pnpm install
pnpm tauri dev     # dev mode with hot reload
pnpm tauri build   # production bundles (MSI/NSIS, dmg, deb/AppImage)
```

## Notes

- **Data lives in** the OS config dir (e.g.
  `%APPDATA%\com.prayertimes.app` / `~/.config/com.prayertimes.app`):
  `settings.json`, `runtime.json` (fired alarms / done marks for today) and
  `cache/` (one timetable file per month + method).
- **Windows dev notifications**: `pnpm tauri dev` may show plain toast
  notifications; installed builds use the proper app identity.
- **Tray icon**: replace the placeholder icons in `src-tauri/icons/` with
  your own (`pnpm tauri icon path/to/icon.png`).
- Times are computed for the **selected city's timezone** (from the API), so
  alarms are correct even if the device clock timezone differs.
