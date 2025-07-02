import type { nil } from '@tb-dev/utils';
import { invoke } from '@tauri-apps/api/core';

export async function isDesktop() {
  return invoke<boolean>('is_desktop');
}

export async function isMobile() {
  return invoke<boolean>('is_mobile');
}

export async function showWindow() {
  return invoke<nil>('show_window');
}
