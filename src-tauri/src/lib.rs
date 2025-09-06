#![feature(try_blocks)]

mod command;
mod error;
mod window;

#[cfg(desktop)]
mod plugin;

use error::BoxResult;
use tauri::{AppHandle, Manager};

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
  #[cfg(desktop)]
  let builder = {
    tauri::Builder::default()
      .plugin(plugin::prevent_default())
      .plugin(plugin::single_instance())
      .plugin(plugin::window_state())
      .plugin(tauri_plugin_process::init())
  };

  #[cfg(mobile)]
  let builder = tauri::Builder::default();

  builder
    .plugin(tauri_plugin_fs::init())
    .plugin(tauri_plugin_dialog::init())
    .plugin(tauri_plugin_persisted_scope::init())
    .plugin(tauri_plugin_pinia::init())
    .plugin(tauri_plugin_vue::init())
    .setup(|app| setup(app.app_handle()))
    .invoke_handler(tauri::generate_handler![command::show_window])
    .run(tauri::generate_context!())
    .expect("failed to start tauri app");
}

fn setup(app: &AppHandle) -> BoxResult<()> {
  #[cfg(desktop)]
  window::desktop::open(app)?;
  #[cfg(mobile)]
  window::mobile::open(app)?;

  Ok(())
}
