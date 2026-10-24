import { writable } from 'svelte/store';
import { api, defaultState } from './api.js';

export const appState = writable({ ...defaultState, loading: true });
export const appError = writable(null);

export async function hydrateAppState() {
  appState.update((state) => ({ ...state, loading: true }));
  try {
    const state = await api.appGetState();
    appState.set({ ...state, loading: false });
    return state;
  } catch (error) {
    const message = error instanceof Error ? error.message : String(error);
    appError.set(message);
    appState.update((state) => ({ ...state, loading: false, lastError: message }));
    throw error;
  }
}

export async function setTheme(theme) {
  const state = await api.appSetTheme(theme);
  appState.set({ ...state, loading: false });
}

export async function setTypography(fontFamily, fontScale) {
  const state = await api.appSetTypography(fontFamily, fontScale);
  appState.set({ ...state, loading: false });
}
