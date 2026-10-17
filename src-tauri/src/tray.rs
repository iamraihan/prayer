//! System tray: status text (next prayer + countdown), quick "mark done",
//! open and quit. The menu is rebuilt (cheaply) whenever the status changes.

use tauri::menu::{MenuBuilder, MenuItem};
use tauri::tray::{MouseButton, MouseButtonState, TrayIconBuilder, TrayIconEvent};
use tauri::{AppHandle, Manager};

pub const TRAY_ID: &str = "prayer-tray";

pub fn build(app: &AppHandle) -> tauri::Result<()> {
    let builder = TrayIconBuilder::with_id(TRAY_ID)
        .tooltip("Prayer Times")
        .show_menu_on_left_click(false)
        .on_menu_event(|app, event| match event.id.as_ref() {
            "action-quit" => app.exit(0),
            "action-open" => crate::windows::show_main(app),
            "action-done" => crate::engine::tray_mark_done(app),
            _ => {}
        })
        .on_tray_icon_event(|tray, event| {
            if let TrayIconEvent::Click {
                button: MouseButton::Left,
                button_state: MouseButtonState::Up,
                ..
            } = event
            {
                crate::windows::show_main(tray.app_handle());
            }
        });

    let builder = match app.default_window_icon() {
        Some(icon) => builder.icon(icon.clone()),
        None => builder,
    };

    builder.build(app)?;
    update(app, "Prayer Times", "Starting…", "Prayer Times", false, "");
    Ok(())
}

/// Rebuilds the tray menu + tooltip. Called on status changes only (the
/// engine caches the previous lines to avoid flicker).
pub fn update(
    app: &AppHandle,
    line1: &str,
    line2: &str,
    tooltip: &str,
    mark_done_enabled: bool,
    mark_done_label: &str,
) {
    let Some(tray) = app.tray_by_id(TRAY_ID) else {
        return;
    };

    let status_next = MenuItem::with_id(app, "status-next", line1, false, None::<&str>);
    let status_ends = MenuItem::with_id(app, "status-ends", line2, false, None::<&str>);
    let mark_done = MenuItem::with_id(
        app,
        "action-done",
        mark_done_label,
        mark_done_enabled,
        None::<&str>,
    );
    let open = MenuItem::with_id(app, "action-open", "Open Prayer Times", true, None::<&str>);
    let quit = MenuItem::with_id(app, "action-quit", "Quit", true, None::<&str>);

    if let (Ok(a), Ok(b), Ok(c), Ok(d), Ok(e)) =
        (status_next, status_ends, mark_done, open, quit)
    {
        if let Ok(menu) = MenuBuilder::new(app)
            .item(&a)
            .item(&b)
            .separator()
            .item(&c)
            .item(&d)
            .separator()
            .item(&e)
            .build()
        {
            let _ = tray.set_menu(Some(menu));
        }
    }

    let _ = tray.set_tooltip(Some(tooltip));
}
