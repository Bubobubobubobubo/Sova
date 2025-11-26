import { writable, derived, get, type Writable, type Readable } from 'svelte/store';
import { listen, type UnlistenFn } from '@tauri-apps/api/event';
import { SERVER_EVENTS } from '$lib/events';
import type { RemoveFramePayload } from '$lib/types/protocol';

export interface LocalEdit {
	content: string;
	lang: string;
}

// Map<frameKey, LocalEdit> where frameKey = "lineIdx-frameIdx"
export const localEdits: Writable<Map<string, LocalEdit>> = writable(new Map());

let unlistenFunctions: UnlistenFn[] = [];

function makeKey(lineIdx: number, frameIdx: number): string {
	return `${lineIdx}-${frameIdx}`;
}

// Get local edit for frame (or null if none)
export function getLocalEdit(frameKey: string): LocalEdit | null {
	return get(localEdits).get(frameKey) ?? null;
}

// Save local edit
export function setLocalEdit(frameKey: string, content: string, lang: string): void {
	localEdits.update(($edits) => {
		const newEdits = new Map($edits);
		newEdits.set(frameKey, { content, lang });
		return newEdits;
	});
}

// Clear local edit (after eval or discard)
export function clearLocalEdit(frameKey: string): void {
	localEdits.update(($edits) => {
		const newEdits = new Map($edits);
		newEdits.delete(frameKey);
		return newEdits;
	});
}

// Clear all local edits
export function clearAllLocalEdits(): void {
	localEdits.set(new Map());
}

// Initialize store with cleanup listeners
export async function initializeLocalEditsStore(): Promise<void> {
	// Clean up local edits when frames are removed
	unlistenFunctions.push(
		await listen<RemoveFramePayload>(SERVER_EVENTS.REMOVE_FRAME, (event) => {
			const { lineId, frameId } = event.payload;
			localEdits.update(($edits) => {
				const newEdits = new Map<string, LocalEdit>();
				for (const [key, value] of $edits.entries()) {
					const [lid, fid] = key.split('-').map(Number);
					if (lid === lineId) {
						if (fid < frameId) {
							newEdits.set(key, value);
						} else if (fid > frameId) {
							// Shift index down
							newEdits.set(makeKey(lid, fid - 1), value);
						}
						// fid === frameId: deleted, skip
					} else {
						newEdits.set(key, value);
					}
				}
				return newEdits;
			});
		})
	);

	// Clean up local edits when lines are removed
	unlistenFunctions.push(
		await listen<number>(SERVER_EVENTS.REMOVE_LINE, (event) => {
			const removedLineId = event.payload;
			localEdits.update(($edits) => {
				const newEdits = new Map<string, LocalEdit>();
				for (const [key, value] of $edits.entries()) {
					const [lid, fid] = key.split('-').map(Number);
					if (lid < removedLineId) {
						newEdits.set(key, value);
					} else if (lid > removedLineId) {
						// Shift line index down
						newEdits.set(makeKey(lid - 1, fid), value);
					}
					// lid === removedLineId: deleted, skip
				}
				return newEdits;
			});
		})
	);
}

// Cleanup on disconnect
export function cleanupLocalEditsStore(): void {
	for (const unlisten of unlistenFunctions) {
		unlisten();
	}
	unlistenFunctions = [];
	localEdits.set(new Map());
}
