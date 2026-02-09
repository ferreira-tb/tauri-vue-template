use crate::error::CResult;

#[cfg(desktop)]
use tauri::WebviewWindow;

#[cfg(desktop)]
#[tauri::command]
pub async fn show_window(window: WebviewWindow) -> CResult<()> {
  window.show()?;
  window.unminimize()?;
  window.set_focus()?;
  Ok(())
}

#[cfg(mobile)]
#[tauri::command]
pub async fn show_window() -> CResult<()> {
  Ok(())
}
