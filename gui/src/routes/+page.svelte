<script lang="ts">
  import { onMount, onDestroy } from 'svelte';
  import ThemeProvider from '$lib/components/ThemeProvider.svelte';
  import TopBar from '$lib/components/TopBar.svelte';
  import Editor from '$lib/components/Editor.svelte';
  import ConfigEditor from '$lib/components/ConfigEditor.svelte';
  import { viewState } from '$lib/stores/viewState';
  import { initializeConfig, cleanupConfigLoader } from '$lib/stores/configLoader';

  let currentView = $state($viewState);

  $effect(() => {
    currentView = $viewState;
  });

  onMount(async () => {
    await initializeConfig();
  });

  onDestroy(() => {
    cleanupConfigLoader();
  });
</script>

<ThemeProvider>
  <div class="app">
    <TopBar {currentView} />
    <div class="content">
      {#if currentView === 'EDITOR'}
        <Editor />
      {:else}
        <ConfigEditor />
      {/if}
    </div>
  </div>
</ThemeProvider>

<style>
  :global(body) {
    margin: 0;
    padding: 0;
    overflow: hidden;
  }

  .app {
    width: 100vw;
    height: 100vh;
    display: flex;
    flex-direction: column;
    background-color: var(--colors-background, #1e1e1e);
    color: var(--colors-text, #ffffff);
  }

  .content {
    flex: 1;
    overflow: hidden;
  }
</style>
