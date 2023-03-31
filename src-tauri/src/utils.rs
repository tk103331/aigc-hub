
pub fn open_app(app : App) {
    let ctx = tauri::generate_context!();
    tauri::Builder::default()
        .setup(|app| {
            Ok(())
        })
        .run(ctx)
        .expect("error while running tauri application");
}