import { writable } from 'svelte/store';

export type ViewType = 'EDITOR' | 'CONFIG' | 'LOGIN';

export const viewState = writable<ViewType>('EDITOR');
