<script lang="ts">
	import { onMount, onDestroy } from 'svelte';
	import { listen, type UnlistenFn } from '@tauri-apps/api/event';
	import { scene } from '$lib/stores';
	import { selection } from '$lib/stores/selection';
	import { SERVER_EVENTS } from '$lib/events';
	import type { Frame, RemoveFramePayload } from '$lib/types/protocol';
	import SplitPane from './SplitPane.svelte';
	import SceneToolbar from './scene/SceneToolbar.svelte';
	import Timeline from './scene/Timeline.svelte';
	import FrameEditor from './scene/FrameEditor.svelte';

	// Zoom constraints
	const MIN_ZOOM = 0.25;
	const MAX_ZOOM = 4.0;
	const ZOOM_FACTOR = 1.1;

	// Viewport state
	let viewport = $state({ zoom: 1.0, orientation: 'horizontal' as 'horizontal' | 'vertical' });

	// Layout state
	let splitOrientation = $state<'horizontal' | 'vertical'>('horizontal');

	// Editor state
	let editingFrame = $state<{ lineIdx: number; frameIdx: number } | null>(null);

	// Derived: get the frame being edited
	const currentFrame = $derived((): Frame | null => {
		if (!editingFrame || !$scene) return null;
		const line = $scene.lines[editingFrame.lineIdx];
		if (!line) return null;
		return line.frames[editingFrame.frameIdx] ?? null;
	});

	const frameKey = $derived(
		editingFrame ? `${editingFrame.lineIdx}-${editingFrame.frameIdx}` : null
	);

	function handleZoomChange(zoom: number) {
		viewport.zoom = zoom;
	}

	function toggleTimelineOrientation() {
		viewport.orientation = viewport.orientation === 'horizontal' ? 'vertical' : 'horizontal';
	}

	function toggleSplitOrientation() {
		splitOrientation = splitOrientation === 'horizontal' ? 'vertical' : 'horizontal';
	}

	function handleOpenEditor(lineIdx: number, frameIdx: number) {
		editingFrame = { lineIdx, frameIdx };
	}

	// Listen for frame/line removal to update editingFrame
	let unlistenFns: UnlistenFn[] = [];

	onMount(async () => {
		unlistenFns.push(
			await listen<RemoveFramePayload>(SERVER_EVENTS.REMOVE_FRAME, (event) => {
				if (!editingFrame) return;
				const { lineId, frameId } = event.payload;
				if (editingFrame.lineIdx === lineId) {
					if (editingFrame.frameIdx === frameId) {
						editingFrame = null; // Deleted the frame we were editing
					} else if (editingFrame.frameIdx > frameId) {
						editingFrame = { lineIdx: lineId, frameIdx: editingFrame.frameIdx - 1 };
					}
				}
			}),
			await listen<number>(SERVER_EVENTS.REMOVE_LINE, (event) => {
				if (!editingFrame) return;
				const removedLineId = event.payload;
				if (editingFrame.lineIdx === removedLineId) {
					editingFrame = null; // Deleted the line we were editing
				} else if (editingFrame.lineIdx > removedLineId) {
					editingFrame = { ...editingFrame, lineIdx: editingFrame.lineIdx - 1 };
				}
			})
		);
	});

	onDestroy(() => {
		unlistenFns.forEach((fn) => fn());
	});
</script>

<div class="scene-container">
	<SceneToolbar
		zoom={viewport.zoom}
		minZoom={MIN_ZOOM}
		maxZoom={MAX_ZOOM}
		zoomFactor={ZOOM_FACTOR}
		orientation={viewport.orientation}
		{splitOrientation}
		onZoomChange={handleZoomChange}
		onOrientationChange={toggleTimelineOrientation}
		onSplitOrientationChange={toggleSplitOrientation}
	/>

	<div class="split-container">
		<SplitPane orientation={splitOrientation}>
			{#snippet first()}
				<Timeline
					{viewport}
					minZoom={MIN_ZOOM}
					maxZoom={MAX_ZOOM}
					zoomFactor={ZOOM_FACTOR}
					onZoomChange={handleZoomChange}
					onOpenEditor={handleOpenEditor}
				/>
			{/snippet}

			{#snippet second()}
				<FrameEditor
					frame={currentFrame()}
					{frameKey}
					lineIdx={editingFrame?.lineIdx ?? null}
					frameIdx={editingFrame?.frameIdx ?? null}
				/>
			{/snippet}
		</SplitPane>
	</div>
</div>

<style>
	.scene-container {
		width: 100%;
		height: 100%;
		display: flex;
		flex-direction: column;
		background-color: var(--colors-background);
	}

	.split-container {
		flex: 1;
		overflow: hidden;
	}
</style>
