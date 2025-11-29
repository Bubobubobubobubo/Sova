import { writable, derived, get, type Writable, type Readable } from 'svelte/store';
import { listen } from '@tauri-apps/api/event';
import type { LinkState, ClockState, FramePosition } from '$lib/types/protocol';
import { ticker } from './ticker';
import { getClock, getScene } from '$lib/api/client';
import { isConnected } from './connectionState';
import { ListenerGroup } from './helpers';

// Transport state
export const isPlaying: Writable<boolean> = writable(false);

// Clock state
export const clockState: Writable<ClockState | null> = writable(null);

// Link state (Ableton Link)
export const linkState: Writable<LinkState | null> = writable(null);

// Frame positions (line_idx, frame_idx)
export const framePositions: Writable<FramePosition[]> = writable([]);

// Derived stores
export const currentTempo: Readable<number | null> = derived(
	clockState,
	($clock) => $clock?.tempo ?? null
);

export const currentBeat: Readable<number | null> = derived(
	clockState,
	($clock) => $clock?.beat ?? null
);

export const currentQuantum: Readable<number | null> = derived(
	clockState,
	($clock) => $clock?.quantum ?? null
);

export const linkTempo: Readable<number | null> = derived(
	linkState,
	($link) => $link?.tempo ?? null
);

export const linkPeerCount: Readable<number | null> = derived(
	linkState,
	($link) => $link?.numPeers ?? null
);

export const isLinkEnabled: Readable<boolean> = derived(
	linkState,
	($link) => $link?.isEnabled ?? false
);

// Helper to get current frame for a specific line
export function getCurrentFrameForLine(lineId: number): Readable<number | null> {
	return derived(framePositions, ($positions) => {
		const position = $positions[lineId];
		return position ? position[0] : null;
	});
}

const listeners = new ListenerGroup();
let unsubscribeTicker: (() => void) | null = null;
let tickCount = 0;

export async function initializeTransportStore(): Promise<void> {
	// Listen for transport started
	await listeners.add(() =>
		listen('server:transport-started', () => {
			isPlaying.set(true);
		})
	);

	// Listen for transport stopped
	await listeners.add(() =>
		listen('server:transport-stopped', () => {
			isPlaying.set(false);
		})
	);

	// Listen for clock state updates
	await listeners.add(() =>
		listen<ClockState>('server:clock-state', (event) => {
			clockState.set(event.payload);
		})
	);

	// Listen for frame position updates
	await listeners.add(() =>
		listen<FramePosition[]>('server:frame-position', (event) => {
			framePositions.set(event.payload);
		})
	);

	// Subscribe to ticker for periodic updates
	// Clock updates every tick (50ms), scene updates every 5 ticks (250ms)
	unsubscribeTicker = ticker.subscribe(() => {
		if (!get(isConnected)) return;

		getClock();

		tickCount++;
		if (tickCount >= 5) {
			getScene();
			tickCount = 0;
		}
	});
}

export function cleanupTransportStore(): void {
	listeners.cleanup();

	if (unsubscribeTicker) {
		unsubscribeTicker();
		unsubscribeTicker = null;
	}

	tickCount = 0;
	isPlaying.set(false);
	clockState.set(null);
	linkState.set(null);
	framePositions.set([]);
}
