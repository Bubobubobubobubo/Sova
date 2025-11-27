<script lang="ts">
	import { paneLayout } from '$lib/stores/paneState';
	import PaneNodeRenderer from './PaneNodeRenderer.svelte';

	let isOnlyPane = $derived($paneLayout.root.type === 'leaf');
</script>

<div class="pane-layout">
	<PaneNodeRenderer node={$paneLayout.root} {isOnlyPane} />
</div>

<style>
	.pane-layout {
		width: 100%;
		height: 100%;
		overflow: hidden;
	}

	.pane-layout :global(.splitpanes) {
		background-color: var(--colors-background);
	}

	.pane-layout :global(.splitpanes__splitter) {
		background-color: var(--colors-border);
		position: relative;
	}

	.pane-layout :global(.splitpanes__splitter:before) {
		content: '';
		position: absolute;
		left: 0;
		top: 0;
		transition: opacity 0.2s;
		background-color: var(--colors-accent);
		opacity: 0;
		z-index: 1;
	}

	.pane-layout :global(.splitpanes__splitter:hover:before) {
		opacity: 0.5;
	}

	.pane-layout :global(.splitpanes--vertical > .splitpanes__splitter) {
		width: 4px;
		min-width: 4px;
	}

	.pane-layout :global(.splitpanes--vertical > .splitpanes__splitter:before) {
		left: -2px;
		right: -2px;
		height: 100%;
		width: auto;
	}

	.pane-layout :global(.splitpanes--horizontal > .splitpanes__splitter) {
		height: 4px;
		min-height: 4px;
	}

	.pane-layout :global(.splitpanes--horizontal > .splitpanes__splitter:before) {
		top: -2px;
		bottom: -2px;
		width: 100%;
		height: auto;
	}

	.pane-layout :global(.splitpanes__pane) {
		overflow: hidden;
	}
</style>
