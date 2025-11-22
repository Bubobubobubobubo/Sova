<script lang="ts">
  import { viewState, type ViewType } from '$lib/stores/viewState';

  interface Props {
    currentView: ViewType;
  }

  let { currentView }: Props = $props();

  function switchView(view: ViewType) {
    viewState.set(view);
  }
</script>

<div class="topbar">
  <div class="tabs">
    <button
      class="tab"
      class:active={currentView === 'EDITOR'}
      onclick={() => switchView('EDITOR')}>
      EDITOR
    </button>
    <button
      class="tab"
      class:active={currentView === 'CONFIG'}
      onclick={() => switchView('CONFIG')}>
      CONFIG
    </button>
  </div>
</div>

<style>
  .topbar {
    width: 100%;
    height: 40px;
    background-color: var(--colors-background, #1e1e1e);
    border-bottom: 1px solid var(--colors-border, #333);
    display: flex;
    align-items: center;
    padding: 0 16px;
  }

  .tabs {
    display: flex;
    gap: 4px;
  }

  .tab {
    background: none;
    border: none;
    color: var(--colors-text-secondary, #888);
    font-family: monospace;
    font-size: 14px;
    font-weight: 600;
    letter-spacing: 0.5px;
    padding: 8px 16px;
    cursor: pointer;
    transition: color 0.2s;
  }

  .tab:hover {
    color: var(--colors-text, #aaa);
  }

  .tab.active {
    color: var(--colors-text, #fff);
    background-color: var(--colors-surface, #2d2d2d);
  }
</style>
