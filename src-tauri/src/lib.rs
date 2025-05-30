#![feature(let_chains, try_blocks)]

mod api;
mod command;
mod error;
mod window;

#[cfg(desktop)]
mod plugin;

use error::BoxResult;
use tauri::{AppHandle, Manager};

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
  let specta = api::collect();

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
    .invoke_handler(specta.invoke_handler())
    .run(tauri::generate_context!())
    .expect("failed to start tauri app");
}

fn setup(app: &AppHandle) -> BoxResult<()> {
  window::open(app)?;
  Ok(())
}
