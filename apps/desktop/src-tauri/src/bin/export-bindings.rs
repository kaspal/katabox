//! Regenerate `apps/desktop/src/bindings.ts` from the Tauri commands (ADR 0011):
//! `cargo run -p katabox-desktop --bin export-bindings`.

fn main() {
    katabox_lib::export_bindings();
    println!("wrote apps/desktop/src/bindings.ts");
}
