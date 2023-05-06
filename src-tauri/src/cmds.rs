use tauri::AppHandle;

use crate::conf;
use crate::utils;
 

#[tauri::command]
pub fn load_app_list(_handle: AppHandle) -> Vec<conf::App> {
    let config = conf::load();
    config.apps
}

#[tauri::command]
pub fn save_app_list(_handle: AppHandle, apps: Vec<conf::App>) {
    let mut config = conf::load();
    config.apps = apps;
    conf::save(&config);
}

#[tauri::command]
pub fn open_app(handle: AppHandle, app_name: &str) {
    
    let config = conf::load();
    config.apps.iter().filter(|app| app.name == app_name).next().map(|app| {
        utils::open_app(&handle, app);
    });
}