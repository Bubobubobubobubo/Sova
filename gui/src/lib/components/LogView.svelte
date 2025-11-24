<script lang="ts">
  import {
    logs,
    filteredLogs,
    showFatal,
    showError,
    showWarn,
    showInfo,
    showDebug
  } from '$lib/stores/logs';
  import type { Severity } from '$lib/types/protocol';

  let scrollContainer: HTMLDivElement;
  let autoScroll = $state(true);

  function formatTimestamp(timestamp: number): string {
    const date = new Date(timestamp);
    const hours = String(date.getHours()).padStart(2, '0');
    const minutes = String(date.getMinutes()).padStart(2, '0');
    const seconds = String(date.getSeconds()).padStart(2, '0');
    const ms = String(date.getMilliseconds()).padStart(3, '0');
    return `${hours}:${minutes}:${seconds}.${ms}`;
  }

  function getSeverityClass(level: Severity | undefined): string {
    if (!level) return 'severity-info'; // Fallback for logs without level
    return `severity-${level.toLowerCase()}`;
  }

  function getLogLevel(log: LogEntry): string {
    return log.level ? log.level.toUpperCase() : 'INFO';
  }

  function clearLogs() {
    logs.set([]);
  }

  function scrollToBottom() {
    if (autoScroll && scrollContainer) {
      scrollContainer.scrollTop = scrollContainer.scrollHeight;
    }
  }

  $effect(() => {
    $filteredLogs;
    scrollToBottom();
  });
</script>

<div class="logs-view">
  <div class="toolbar">
    <h2 class="title">LOGS</h2>
    <div class="toolbar-actions">
      <div class="filter-group">
        <span class="filter-label">Show:</span>
        <label class="filter-toggle">
          <input type="checkbox" bind:checked={$showFatal} />
          Fatal
        </label>
        <label class="filter-toggle">
          <input type="checkbox" bind:checked={$showError} />
          Error
        </label>
        <label class="filter-toggle">
          <input type="checkbox" bind:checked={$showWarn} />
          Warn
        </label>
        <label class="filter-toggle">
          <input type="checkbox" bind:checked={$showInfo} />
          Info
        </label>
        <label class="filter-toggle">
          <input type="checkbox" bind:checked={$showDebug} />
          Debug
        </label>
      </div>
      <label class="auto-scroll-toggle">
        <input type="checkbox" bind:checked={autoScroll} />
        Auto-scroll
      </label>
      <button class="clear-button" onclick={clearLogs}>
        Clear
      </button>
    </div>
  </div>

  <div class="logs-content" bind:this={scrollContainer}>
    {#if $filteredLogs.length === 0}
      <div class="empty-state">
        {$logs.length === 0 ? 'No logs yet' : 'No logs match current filters'}
      </div>
    {:else}
      <div class="logs-list">
        {#each $filteredLogs as log}
          <div class="log-entry {getSeverityClass(log.level)}">
            <span class="log-level">[{getLogLevel(log)}]</span>
            <span class="log-timestamp">{formatTimestamp(log.timestamp)}</span>
            <span class="log-message">{log.message}</span>
          </div>
        {/each}
      </div>
    {/if}
  </div>
</div>

<style>
  .logs-view {
    width: 100%;
    height: 100%;
    display: flex;
    flex-direction: column;
    background-color: var(--colors-background);
  }

  .toolbar {
    height: 40px;
    background-color: var(--colors-surface, #252525);
    border-bottom: 1px solid var(--colors-border, #333);
    display: flex;
    align-items: center;
    justify-content: space-between;
    padding: 0 16px;
  }

  .title {
    margin: 0;
    font-size: 13px;
    font-weight: 600;
    color: var(--colors-text, #fff);
    font-family: monospace;
  }

  .toolbar-actions {
    display: flex;
    align-items: center;
    gap: 16px;
  }

  .filter-group {
    display: flex;
    align-items: center;
    gap: 8px;
    padding: 4px 12px;
    background-color: var(--colors-background, #1e1e1e);
    border: 1px solid var(--colors-border, #333);
  }

  .filter-label {
    font-size: 12px;
    color: var(--colors-text-secondary, #888);
    font-family: monospace;
    font-weight: 600;
  }

  .filter-toggle {
    display: flex;
    align-items: center;
    gap: 4px;
    font-size: 12px;
    color: var(--colors-text-secondary, #888);
    font-family: monospace;
    cursor: pointer;
  }

  .filter-toggle input[type="checkbox"] {
    cursor: pointer;
  }

  .auto-scroll-toggle {
    display: flex;
    align-items: center;
    gap: 6px;
    font-size: 13px;
    color: var(--colors-text-secondary, #888);
    font-family: monospace;
    cursor: pointer;
  }

  .auto-scroll-toggle input[type="checkbox"] {
    cursor: pointer;
  }

  .clear-button {
    background-color: var(--colors-accent, #0e639c);
    color: var(--colors-text, #fff);
    border: none;
    padding: 4px 12px;
    font-size: 13px;
    cursor: pointer;
    font-family: monospace;
  }

  .clear-button:hover {
    background-color: var(--colors-accent-hover, #1177bb);
  }

  .logs-content {
    flex: 1;
    overflow-y: auto;
    overflow-x: hidden;
    padding: 16px;
  }

  .empty-state {
    color: var(--colors-text-secondary, #888);
    font-family: monospace;
    font-size: 13px;
    text-align: center;
    padding: 32px;
  }

  .logs-list {
    display: flex;
    flex-direction: column;
    gap: 2px;
  }

  .log-entry {
    display: flex;
    gap: 12px;
    font-family: monospace;
    font-size: 13px;
    padding: 4px 8px;
    border-bottom: 1px solid var(--colors-border, #333);
  }

  .log-entry:hover {
    background-color: var(--colors-surface, #2d2d2d);
  }

  .log-level {
    flex-shrink: 0;
    min-width: 70px;
    font-weight: 600;
    font-size: 11px;
  }

  .log-timestamp {
    color: var(--colors-text-secondary, #888);
    flex-shrink: 0;
    min-width: 90px;
  }

  .log-message {
    color: var(--colors-text, #fff);
    flex: 1;
    word-break: break-word;
  }

  /* Severity color coding */
  .severity-fatal .log-level {
    color: #ff3366;
  }

  .severity-error .log-level {
    color: #ff6b6b;
  }

  .severity-warn .log-level {
    color: #ffa500;
  }

  .severity-info .log-level {
    color: #4ecdc4;
  }

  .severity-debug .log-level {
    color: #95a5a6;
  }
</style>
