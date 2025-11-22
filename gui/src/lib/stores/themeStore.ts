import { writable, derived, type Writable, type Readable } from 'svelte/store';
import { listen, type UnlistenFn } from '@tauri-apps/api/event';
import { themes, type Theme } from '$lib/themes';

export const currentThemeName: Writable<string> = writable('monokai');

export const currentTheme: Readable<Theme> = derived(currentThemeName, ($name) => {
  const theme = themes[$name];
  if (!theme) {
    throw new Error(`Invalid theme "${$name}" specified in config. Available themes: ${Object.keys(themes).slice(0, 10).join(', ')}...`);
  }
  return theme;
});

let unlisten: UnlistenFn | null = null;

export async function initThemeListener() {
  if (unlisten) return;

  unlisten = await listen<{ appearance: { theme: string } }>(
    'config-update',
    (event) => {
      const themeName = event.payload.appearance.theme;
      console.log('Theme updated:', themeName);
      currentThemeName.set(themeName);
    }
  );
}

export function cleanupThemeListener() {
  if (unlisten) {
    unlisten();
    unlisten = null;
  }
}
