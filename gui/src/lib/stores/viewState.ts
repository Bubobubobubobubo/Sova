import { writable } from 'svelte/store';

export type ViewType = 'EDITOR' | 'CONFIG';

export const viewState = writable<ViewType>('EDITOR');
