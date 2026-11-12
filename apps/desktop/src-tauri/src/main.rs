// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

fn main() -> std::process::ExitCode {
    // On Linux the app binary is also the sandbox helper: kb-runner starts it with a
    // hidden `__*` subcommand for every run (ADR 0007).
    #[cfg(target_os = "linux")]
    {
        let args: Vec<String> = std::env::args().collect();
        if let Some(cmd) = args.get(1) {
            if let Some(code) = kb_runner::helper_main(cmd, &args[2..]) {
                return code;
            }
        }
    }
    // The installers' uninstall step (ADR 0021): revoke the sandbox grants and delete the
    // AppContainer profile, then exit without opening a window.
    if std::env::args().nth(1).as_deref() == Some("--uninstall-cleanup") {
        return katabox_lib::uninstall_cleanup();
    }
    katabox_lib::run_app();
    std::process::ExitCode::SUCCESS
}
