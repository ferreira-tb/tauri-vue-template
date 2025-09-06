import type { nil } from '@tb-dev/utils';
import { invoke } from '@tauri-apps/api/core';

export async function showWindow() {
  return invoke<nil>('show_window');
}
