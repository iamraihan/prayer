//! Small window helpers.

use tauri::{AppHandle, Manager};

/// Shows and focuses the main window (tray click / "Open" menu item).
pub fn show_main(app: &AppHandle) {
    if let Some(win) = app.get_webview_window("main") {
        let _ = win.unminimize();
        let _ = win.show();
        let _ = win.set_focus();
    }
}
