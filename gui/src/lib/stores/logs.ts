import { writable, derived, type Writable, type Readable } from 'svelte/store';
import { listen, type UnlistenFn } from '@tauri-apps/api/event';
import { SERVER_EVENTS } from '$lib/events';
import type { LogMessage, Severity } from '$lib/types/protocol';

export interface LogEntry {
	message: string;
	level: Severity;
	timestamp: number;
}

const MAX_LOGS = 1000; // Circular buffer size

// Log entries in chronological order (newest last)
export const logs: Writable<LogEntry[]> = writable([]);

// Filter settings - which severity levels to show (default: all except Debug)
export const showFatal = writable(true);
export const showError = writable(true);
export const showWarn = writable(true);
export const showInfo = writable(true);
export const showDebug = writable(false); // Hide debug logs by default (includes GetClock queries)

// Derived store for filtered logs
export const filteredLogs: Readable<LogEntry[]> = derived(
	[logs, showFatal, showError, showWarn, showInfo, showDebug],
	([$logs, $showFatal, $showError, $showWarn, $showInfo, $showDebug]) => {
		return $logs.filter((log) => {
			switch (log.level) {
				case 'Fatal':
					return $showFatal;
				case 'Error':
					return $showError;
				case 'Warn':
					return $showWarn;
				case 'Info':
					return $showInfo;
				case 'Debug':
					return $showDebug;
				default:
					return true;
			}
		});
	}
);

// Derived store for recent filtered logs
export const recentLogs: Readable<LogEntry[]> = derived(filteredLogs, ($filteredLogs) =>
	$filteredLogs.slice(-100)
);

// Pure function to add log with circular buffer logic
function addLog(currentLogs: LogEntry[], logMessage: LogMessage): LogEntry[] {
	const newLog: LogEntry = {
		message: logMessage.msg,
		level: logMessage.level,
		timestamp: Date.now()
	};
	const updated = [...currentLogs, newLog];

	// Keep only last MAX_LOGS entries
	return updated.length > MAX_LOGS ? updated.slice(-MAX_LOGS) : updated;
}

let unlistenFunctions: UnlistenFn[] = [];
let isInitialized = false;

export async function initializeLogsStore(): Promise<void> {
	if (isInitialized) {
		return;
	}

	unlistenFunctions.push(
		await listen<LogMessage>(SERVER_EVENTS.LOG, (event) => {
			logs.update(($logs) => addLog($logs, event.payload));
		})
	);

	isInitialized = true;
}

export function cleanupLogsStore(): void {
	for (const unlisten of unlistenFunctions) {
		unlisten();
	}
	unlistenFunctions = [];
	isInitialized = false;
}

// Export for testing
export { addLog };
