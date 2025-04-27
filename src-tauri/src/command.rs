use crate::error::CResult;

#[cfg(desktop)]
use tauri::WebviewWindow;

#[cfg(desktop)]
#[tauri::command]
#[specta::specta]
pub async fn show_window(window: WebviewWindow) -> CResult<()> {
  window
    .show()
    .and_then(|()| window.unminimize())
    .and_then(|()| window.set_focus())
    .map_err(Into::into)
}

#[cfg(mobile)]
#[tauri::command]
#[specta::specta]
pub async fn show_window() -> CResult<()> {
  Ok(())
}
