use anyhow::Result;
use tauri::{AppHandle, WebviewUrl, WebviewWindowBuilder};

#[cfg(desktop)]
use tauri::{Manager, WebviewWindow, Wry};

#[cfg(desktop)]
pub trait WindowExt: Manager<Wry> {
  fn main_window(&self) -> WebviewWindow<Wry> {
    self.get_webview_window("main").unwrap()
  }
}

#[cfg(desktop)]
impl<T: Manager<Wry>> WindowExt for T {}

#[cfg(desktop)]
pub fn open(app: &AppHandle) -> Result<()> {
  WebviewWindowBuilder::new(app, "main", url())
    .title("tauri-vue-template")
    .inner_size(1200.0, 800.0)
    .resizable(true)
    .maximizable(true)
    .minimizable(true)
    .visible(false)
    .prevent_overflow()
    .build()?;

  Ok(())
}

#[cfg(mobile)]
pub fn open(app: &AppHandle) -> Result<()> {
  WebviewWindowBuilder::new(app, "main", url()).build()?;
  Ok(())
}

fn url() -> WebviewUrl {
  WebviewUrl::App("index.html".into())
}
