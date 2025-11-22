import { writable, type Writable } from 'svelte/store';
import { listen, type UnlistenFn } from '@tauri-apps/api/event';

export interface EditorConfig {
  mode: 'vim' | 'normal' | 'emacs';
  font_size: number;
  show_line_numbers: boolean;
  line_wrapping: boolean;
  highlight_active_line: boolean;
  cursor_blink_rate: number;
  tab_size: number;
  indent_unit: string;
  use_tabs: boolean;
  close_brackets: boolean;
  bracket_matching: boolean;
  autocomplete: boolean;
  rectangular_selection: boolean;
  fold_gutter: boolean;
  match_highlighting: boolean;
}

export const editorConfig: Writable<EditorConfig> = writable({
  mode: 'normal',
  font_size: 14,
  show_line_numbers: true,
  line_wrapping: false,
  highlight_active_line: true,
  cursor_blink_rate: 1200,
  tab_size: 4,
  indent_unit: '  ',
  use_tabs: false,
  close_brackets: true,
  bracket_matching: true,
  autocomplete: true,
  rectangular_selection: true,
  fold_gutter: true,
  match_highlighting: true
});

let unlisten: UnlistenFn | null = null;

export async function initConfigListener() {
  if (unlisten) return;

  unlisten = await listen<{ editor: EditorConfig }>(
    'config-update',
    (event) => {
      console.log('Config updated:', event.payload.editor);
      editorConfig.set(event.payload.editor);
    }
  );
}

export function cleanupConfigListener() {
  if (unlisten) {
    unlisten();
    unlisten = null;
  }
}
