import { invoke } from '@tauri-apps/api/core';
import { open } from '@tauri-apps/plugin-dialog';
import type { ProjectInfo } from '$lib/types/projects';
import type { Snapshot, DeviceMapSnapshot, LoadedProject } from '$lib/types/protocol';

export async function listProjects(): Promise<ProjectInfo[]> {
	return invoke<ProjectInfo[]>('list_projects');
}

export async function importProject(): Promise<LoadedProject | null> {
	const path = await open({
		title: 'Import Snapshot',
		filters: [{ name: 'Sova Snapshot', extensions: ['sova'] }]
	});

	if (!path) return null;

	return invoke<LoadedProject>('import_project', { path });
}

export async function saveProject(
	snapshot: Snapshot,
	deviceConfig: DeviceMapSnapshot | null,
	projectName: string
): Promise<void> {
	return invoke('save_project', { snapshot, deviceConfig, projectName });
}

export async function loadProject(projectName: string): Promise<LoadedProject> {
	return invoke<LoadedProject>('load_project', { projectName });
}

export async function deleteProject(projectName: string): Promise<void> {
	return invoke('delete_project', { projectName });
}

export async function renameProject(oldName: string, newName: string): Promise<void> {
	return invoke('rename_project', { oldName, newName });
}

export async function openProjectsFolder(): Promise<void> {
	return invoke('open_projects_folder');
}
