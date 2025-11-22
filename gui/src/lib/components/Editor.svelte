<script lang="ts">
  import { onMount, onDestroy } from 'svelte';
  import { javascript } from '@codemirror/lang-javascript';
  import { editorConfig } from '$lib/stores/editorConfig';
  import { currentTheme } from '$lib/stores/themeStore';
  import { createEditor, createEditorSubscriptions } from '$lib/editor/editorFactory';
  import type { EditorView } from '@codemirror/view';

  let editorContainer: HTMLDivElement;
  let editorView: EditorView | null = null;
  let unsubscribe: (() => void) | null = null;

  onMount(async () => {
    const config = $editorConfig;
    const theme = $currentTheme;

    editorView = createEditor(
      editorContainer,
      '// Start coding...\n',
      javascript(),
      config,
      theme
    );

    unsubscribe = createEditorSubscriptions(editorView);
  });

  onDestroy(() => {
    if (unsubscribe) {
      unsubscribe();
    }
    editorView?.destroy();
  });
</script>

<div class="editor-wrapper">
  <div class="editor-container" bind:this={editorContainer}></div>
</div>

<style>
  .editor-wrapper {
    width: 100%;
    height: 100%;
    overflow: hidden;
  }

  .editor-container {
    width: 100%;
    height: 100%;
  }

  :global(.cm-editor) {
    height: 100%;
  }
</style>
