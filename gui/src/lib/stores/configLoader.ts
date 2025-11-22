import { invoke } from '@tauri-apps/api/core';
import { listen, type UnlistenFn } from '@tauri-apps/api/event';
import { editorConfig, type EditorConfig } from './editorConfig';
import { currentThemeName, currentTransparency } from './themeStore';

interface Config {
  editor: EditorConfig;
  appearance: {
    theme: string;
    transparency: number;
  };
}

interface ConfigUpdateEvent {
  editor: EditorConfig;
  appearance: {
    theme: string;
    transparency: number;
  };
}

let unlisten: UnlistenFn | null = null;

export async function initializeConfig(): Promise<void> {
  try {
    const config = await invoke<Config>('get_config');

    editorConfig.set(config.editor);
    currentThemeName.set(config.appearance.theme);
    currentTransparency.set(config.appearance.transparency);

    console.log('Config loaded on startup:', config);
  } catch (error) {
    console.error('Failed to load initial config:', error);
  }

  if (unlisten) return;

  unlisten = await listen<ConfigUpdateEvent>(
    'config-update',
    (event) => {
      console.log('Config updated via event:', event.payload);
      editorConfig.set(event.payload.editor);
      currentThemeName.set(event.payload.appearance.theme);
      currentTransparency.set(event.payload.appearance.transparency);
    }
  );
}

export function cleanupConfigLoader() {
  if (unlisten) {
    unlisten();
    unlisten = null;
  }
}
