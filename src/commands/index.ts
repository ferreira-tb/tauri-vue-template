import { invoke } from '@tauri-apps/api/core';

export async function showWindow() {
  await invoke('show_window');
}
