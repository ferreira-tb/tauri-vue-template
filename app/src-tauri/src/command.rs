use crate::error::CmdResult;

#[cfg(desktop)]
use tauri::WebviewWindow;

#[cfg(desktop)]
#[tauri::command]
#[specta::specta]
pub async fn show_window(window: WebviewWindow) -> CmdResult<()> {
  window.show()?;
  window.unminimize()?;
  window.set_focus()?;
  Ok(())
}

#[cfg(mobile)]
#[tauri::command]
#[specta::specta]
pub async fn show_window() -> CmdResult<()> {
  Ok(())
}
