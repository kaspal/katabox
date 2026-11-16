//! Window memory (ui-polish): reads the main window and the monitors, and applies what
//! `kb_app::window` decides. No plugin and no capability: the bounds go through the
//! ordinary settings store (`window.bounds`, physical pixels, never exported).
//!
//! - On start the window is hidden (`visible: false` in tauri.conf.json); [`restore`]
//!   places it, maximises it if it was, then shows it.
//! - Moves and resizes are saved 500 ms after the last one, by one saver thread; closing
//!   saves at once. Minimised bounds are never saved.

use std::sync::mpsc::{self, RecvTimeoutError, Sender};
use std::sync::{Arc, Mutex};
use std::time::Duration;

use kb_app::window::{
    self as rules, Monitor, Placement, SizeRules, WINDOW_BOUNDS_KEY, WindowBounds, WindowNow,
};
use tauri::{Manager, PhysicalPosition, PhysicalSize, WebviewWindow, WindowEvent};

use crate::AppState;

const DEBOUNCE: Duration = Duration::from_millis(500);

/// The last bounds saved (or restored), so unchanged moves write nothing.
#[derive(Default)]
pub struct Saved(Mutex<Option<WindowBounds>>);

/// The logical default and minimum sizes of the main window, from tauri.conf.json.
fn size_rules(window: &WebviewWindow) -> SizeRules {
    let config = window.config().app.windows.first();
    let pair = |w: Option<f64>, h: Option<f64>, fallback: (u32, u32)| match (w, h) {
        (Some(w), Some(h)) if w > 0.0 && h > 0.0 => (w.round() as u32, h.round() as u32),
        _ => fallback,
    };
    SizeRules {
        default: pair(
            config.map(|c| c.width),
            config.map(|c| c.height),
            (1280, 800),
        ),
        min: pair(
            config.and_then(|c| c.min_width),
            config.and_then(|c| c.min_height),
            (960, 600),
        ),
    }
}

fn monitors(window: &WebviewWindow) -> Vec<Monitor> {
    let primary = window.primary_monitor().ok().flatten();
    let is_primary = |m: &tauri::Monitor| {
        primary
            .as_ref()
            .is_some_and(|p| p.position() == m.position() && p.size() == m.size())
    };
    window
        .available_monitors()
        .unwrap_or_default()
        .iter()
        .map(|m| {
            let area = m.work_area();
            Monitor {
                x: area.position.x,
                y: area.position.y,
                width: area.size.width,
                height: area.size.height,
                scale: m.scale_factor(),
                primary: is_primary(m),
            }
        })
        .collect()
}

fn read_now(window: &WebviewWindow) -> Option<WindowNow> {
    let pos = window.outer_position().ok()?;
    let size = window.inner_size().ok()?;
    Some(WindowNow {
        x: pos.x,
        y: pos.y,
        width: size.width,
        height: size.height,
        scale: window.scale_factor().ok()?,
        maximized: window.is_maximized().unwrap_or(false),
        minimized: window.is_minimized().unwrap_or(false),
    })
}

fn apply(window: &WebviewWindow, p: Placement) {
    if window.is_maximized().unwrap_or(false) {
        let _ = window.unmaximize();
    }
    // Position first: moving to a monitor with another scale factor may resize the window.
    let _ = window.set_position(PhysicalPosition::new(p.x, p.y));
    let _ = window.set_size(PhysicalSize::new(p.width, p.height));
    if p.maximized {
        let _ = window.maximize();
    }
}

fn app_of(window: &WebviewWindow) -> Option<tauri::State<'_, AppState>> {
    window.try_state::<AppState>()
}

/// Place the hidden main window from the saved bounds, then show it. Always shows the
/// window, whatever fails.
pub fn restore(window: &WebviewWindow) {
    let saved = app_of(window)
        .and_then(|s| {
            s.app()
                .ok()
                .map(|app| tauri::async_runtime::block_on(app.get_setting(WINDOW_BOUNDS_KEY)))
        })
        .and_then(Result::ok)
        .and_then(rules::parse_bounds);
    if let Some(p) = rules::placement(saved, &monitors(window), size_rules(window)) {
        apply(window, p);
    }
    if let Some(s) = window.try_state::<Saved>() {
        *s.0.lock().unwrap_or_else(|e| e.into_inner()) = saved;
    }
    let _ = window.show();
    let _ = window.set_focus();
}

/// Save the window's bounds now, if they changed and the window isn't minimised.
pub fn save_now(window: &WebviewWindow) {
    let (Some(now), Some(saved), Some(state)) = (
        read_now(window),
        window.try_state::<Saved>(),
        app_of(window),
    ) else {
        return;
    };
    let Ok(app) = state.app() else { return };
    let mut last = saved.0.lock().unwrap_or_else(|e| e.into_inner());
    if let Some(next) = rules::next_bounds(*last, now)
        && let Ok(value) = serde_json::to_value(next)
        && tauri::async_runtime::block_on(app.set_setting(WINDOW_BOUNDS_KEY, &value)).is_ok()
    {
        *last = Some(next);
    }
}

/// Settings → Appearance → "Reset window size and position": the default size, centred on
/// the window's current monitor, saved as the new bounds.
pub fn reset(window: &WebviewWindow) {
    let monitor = window
        .current_monitor()
        .ok()
        .flatten()
        .map(|m| {
            let area = m.work_area();
            Monitor {
                x: area.position.x,
                y: area.position.y,
                width: area.size.width,
                height: area.size.height,
                scale: m.scale_factor(),
                primary: false,
            }
        })
        .or_else(|| {
            let all = monitors(window);
            all.iter().find(|m| m.primary).or(all.first()).copied()
        });
    if let Some(m) = monitor {
        apply(window, rules::reset_placement(&m, size_rules(window)));
    }
    save_now(window);
}

/// Watch the main window: debounced saves on move and resize, an immediate save on close.
pub fn watch(window: &WebviewWindow) {
    let (tx, rx) = mpsc::channel::<()>();
    let saver = window.clone();
    std::thread::Builder::new()
        .name("window-state".into())
        .spawn(move || {
            while rx.recv().is_ok() {
                // Wait until the window has been still for DEBOUNCE.
                loop {
                    match rx.recv_timeout(DEBOUNCE) {
                        Ok(()) => continue,
                        Err(RecvTimeoutError::Timeout) => break,
                        Err(RecvTimeoutError::Disconnected) => return,
                    }
                }
                save_now(&saver);
            }
        })
        .ok();
    let tx: Arc<Mutex<Sender<()>>> = Arc::new(Mutex::new(tx));
    let closing = window.clone();
    window.on_window_event(move |event| match event {
        WindowEvent::Moved(_) | WindowEvent::Resized(_) => {
            let _ = tx.lock().map(|t| t.send(()));
        }
        WindowEvent::CloseRequested { .. } => save_now(&closing),
        _ => {}
    });
}
