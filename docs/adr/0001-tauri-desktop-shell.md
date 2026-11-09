# 1. Tauri 2 as the desktop shell

Date: 2026-11-08 · Status: accepted

## Context
KataBox must be a native Windows and Linux app, small to download, with a rich editor UI and a backend that controls processes and sandboxes.

## Decision
Tauri 2: Rust core plus the system WebView (WebView2 on Windows, WebKitGTK 4.1 on Linux), React 19 + TypeScript UI.

## Consequences
Installer around 10–15 MB instead of ~150 MB for Electron. Rust is a natural fit for sandboxing. We must test on WebKitGTK quirks (NVIDIA/Wayland) and ship a compatibility mode.
