//! The startup splash (ADR 0041). The rules are in `kb_app::splash`; this applies them.
//!
//! - The `splash` window is declared in tauri.conf.json with `create: false`; [`start`]
//!   builds it only when the plan shows it (so in `never` mode it never paints).
//! - The main window starts hidden and placed; the UI calls `app_ready` once after its
//!   first render, and [`ready`] hands over: wait out the animation when the plan says
//!   so, show the main window, fade the splash and close it.
//! - A failsafe hands over after 15 s whatever happens, so the main window always shows.
//! - The splash page has no IPC: it is in no capability, ships no Tauri API, and the fade
//!   is started from here with `eval` (no permission needed).

use std::sync::atomic::{AtomicBool, Ordering};
use std::time::{Duration, Instant};

use kb_app::splash::{
    self as rules, FADE_MS, FAILSAFE_MS, SPLASH_MODE_KEY, SPLASH_VERSION_KEY, SplashPlan,
};
use serde_json::Value;
use tauri::{AppHandle, Manager, WebviewWindowBuilder};

use crate::{AppState, window_state};

pub const SPLASH_LABEL: &str = "splash";

/// One handover per run, whoever comes first: `app_ready` or the failsafe.
pub struct Handoff {
    started: Instant,
    plan: SplashPlan,
    done: AtomicBool,
}

fn read_setting(app: &AppHandle, key: &str) -> Option<Value> {
    let state = app.try_state::<AppState>()?;
    let a = state.app().ok()?;
    tauri::async_runtime::block_on(a.get_setting(key))
        .ok()
        .flatten()
}

/// At setup: decide, record this version, open the splash when the plan shows it, and arm
/// the failsafe.
pub fn start(app: &AppHandle) {
    let current = app.package_info().version.to_string();
    let mode = rules::parse_mode(read_setting(app, SPLASH_MODE_KEY).as_ref());
    let last = read_setting(app, SPLASH_VERSION_KEY);
    let plan = rules::splash_plan(mode, last.as_ref().and_then(Value::as_str), &current);
    if let Some(state) = app.try_state::<AppState>()
        && let Ok(a) = state.app()
    {
        let _ = tauri::async_runtime::block_on(
            a.set_setting(SPLASH_VERSION_KEY, &Value::String(current)),
        );
    }
    let mut plan = plan;
    if plan.show {
        let config = app
            .config()
            .app
            .windows
            .iter()
            .find(|w| w.label == SPLASH_LABEL)
            .cloned();
        let built = config.and_then(|c| {
            WebviewWindowBuilder::from_config(app, &c)
                .ok()?
                .build()
                .ok()
        });
        if built.is_none() {
            eprintln!("KataBox: the splash window could not open; starting without it");
            plan.show = false;
        }
    }
    app.manage(Handoff {
        started: Instant::now(),
        plan,
        done: AtomicBool::new(false),
    });
    let handle = app.clone();
    std::thread::Builder::new()
        .name("splash-failsafe".into())
        .spawn(move || {
            std::thread::sleep(Duration::from_millis(FAILSAFE_MS));
            let pending = handle
                .try_state::<Handoff>()
                .is_some_and(|h| !h.done.load(Ordering::SeqCst));
            if pending {
                eprintln!("KataBox warning: app_ready did not arrive within {FAILSAFE_MS} ms; showing the window");
                handoff(&handle);
            }
        })
        .ok();
}

/// `app_ready` from the main window: hand over (blocking; run it off the main thread).
pub fn ready(app: &AppHandle) {
    // Dev-only: KATABOX_DEV_APP_READY=ignore (test the failsafe) or delay:<ms>. Compiled
    // out of release builds.
    #[cfg(debug_assertions)]
    if let Ok(flag) = std::env::var("KATABOX_DEV_APP_READY") {
        if flag == "ignore" {
            eprintln!("KataBox dev: ignoring app_ready (KATABOX_DEV_APP_READY=ignore)");
            return;
        }
        if let Some(ms) = flag
            .strip_prefix("delay:")
            .and_then(|n| n.parse::<u64>().ok())
        {
            std::thread::sleep(Duration::from_millis(ms));
        }
    }
    handoff(app);
}

fn handoff(app: &AppHandle) {
    let Some(h) = app.try_state::<Handoff>() else {
        // No splash state (setup failed early): just show the window.
        if let Some(main) = app.get_webview_window("main") {
            window_state::show(&main);
        }
        return;
    };
    if h.done.swap(true, Ordering::SeqCst) {
        return;
    }
    let elapsed = u64::try_from(h.started.elapsed().as_millis()).unwrap_or(u64::MAX);
    std::thread::sleep(Duration::from_millis(rules::remaining_ms(h.plan, elapsed)));
    if let Some(main) = app.get_webview_window("main") {
        window_state::show(&main);
    }
    if let Some(splash) = app.get_webview_window(SPLASH_LABEL) {
        let _ = splash.eval("document.documentElement.classList.add('out')");
        std::thread::sleep(Duration::from_millis(FADE_MS));
        let _ = splash.close();
    }
}
