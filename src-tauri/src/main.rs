#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]


fn main() {
    let ctx = tauri::generate_context!();
    tauri::Builder::default()
        .setup(|_app| {
            Ok(())
        })
        .run(ctx)
        .expect("error while running tauri application");
}
