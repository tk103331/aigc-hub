#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]


pub mod conf;
pub mod cmds;
pub mod utils;

fn main() {
    conf::init();
 
    let ctx = tauri::generate_context!();
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            cmds::load_app_list,
            cmds::save_app_list,
            cmds::open_app
        ])
        .setup(|_app| {
            Ok(())
        })
        .run(ctx)
        .expect("error while running tauri application");
}
