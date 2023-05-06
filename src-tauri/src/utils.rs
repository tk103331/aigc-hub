use std::path::PathBuf;

use tauri::{WindowBuilder, WindowUrl};

use crate::conf;

 
pub fn open_app(handle : &tauri::AppHandle, app: &conf::App) {
    WindowBuilder::new(
        handle,
        app.name.clone(),
        WindowUrl::App(PathBuf::from(app.url.clone())),
      )
      .title(app.name.clone())
      .resizable(true)
      .fullscreen(false)
      .inner_size(1200.0, 700.0)
      .min_inner_size(1000.0, 600.0)
      .build()
      .unwrap();
}

pub fn app_home() -> PathBuf {
    tauri::api::path::home_dir().unwrap().join(".aigc-hub")
}

  