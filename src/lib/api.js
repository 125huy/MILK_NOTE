import { invoke, isTauri } from '@tauri-apps/api/core';

const defaultState = {
  vaultPath: null,
  theme: 'paper',
  fontFamily: 'be-vietnam-pro',
  fontScale: 100,
  paperLines: true,
  lastError: null
};

let mockState = { ...defaultState };
const mockNotes = new Map();

const mockApi = {
  async appGetState() { return { ...mockState }; },
  async appSetTheme(theme) { mockState = { ...mockState, theme }; return { ...mockState }; },
  async appSetTypography(fontFamily, fontScale) { mockState = { ...mockState, fontFamily, fontScale }; return { ...mockState }; },
  async vaultOpen(path) { mockState = { ...mockState, vaultPath: path, lastError: null }; return { ...mockState }; },
  async noteRead(path) { return { path, content: mockNotes.get(path) ?? '' }; },
  async noteCreate(path, content) { if (mockNotes.has(path)) throw 'ghi chú đã tồn tại'; mockNotes.set(path, content); return { path, content }; },
  async noteUpdate(path, content) { mockNotes.set(path, content); return { path, content }; }
};

const tauriApi = {
  appGetState: () => invoke('app_get_state'),
  appSetTheme: (theme) => invoke('app_set_theme', { theme }),
  appSetTypography: (fontFamily, fontScale) => invoke('app_set_typography', { fontFamily, fontScale }),
  vaultOpen: (path) => invoke('vault_open', { path }),
  noteRead: (path) => invoke('note_read', { path }),
  noteCreate: (path, content) => invoke('note_create', { path, content }),
  noteUpdate: (path, content, expectedHash = null) => invoke('note_update', { path, content, expectedHash })
};

export const api = isTauri() ? tauriApi : mockApi;
export const runtime = isTauri() ? 'tauri' : 'mock';
export { defaultState };
