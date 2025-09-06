use anyhow::Result;
use tauri::{AppHandle, Manager, WebviewWindow, WebviewWindowBuilder, Wry};

pub trait WindowExt: Manager<Wry> {
  fn main_window(&self) -> WebviewWindow<Wry> {
    self.get_webview_window("main").unwrap()
  }
}

impl<T: Manager<Wry>> WindowExt for T {}

pub fn open(app: &AppHandle) -> Result<()> {
  WebviewWindowBuilder::new(app, "main", super::url())
    .title("tauri-vue-template")
    .initialization_script(super::script())
    .inner_size(1200.0, 800.0)
    .resizable(true)
    .maximizable(true)
    .minimizable(true)
    .visible(false)
    .prevent_overflow()
    .build()?;

  Ok(())
}
