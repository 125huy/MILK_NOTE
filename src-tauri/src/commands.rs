use std::sync::Mutex;

use milknote_core::{storage::WriteOptions, Vault};
use serde::{Deserialize, Serialize};
use tauri::State;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AppSnapshot {
    pub vault_path: Option<String>,
    pub theme: String,
    pub font_family: String,
    pub font_scale: u8,
    pub paper_lines: bool,
    pub last_error: Option<String>,
}

impl Default for AppSnapshot {
    fn default() -> Self {
        Self {
            vault_path: None,
            theme: "paper".into(),
            font_family: "be-vietnam-pro".into(),
            font_scale: 100,
            paper_lines: true,
            last_error: None,
        }
    }
}

#[derive(Default)]
pub struct AppState {
    pub snapshot: Mutex<AppSnapshot>,
    pub vault: Mutex<Option<Vault>>,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct NotePayload {
    pub path: String,
    pub content: String,
}

fn state_error(message: impl Into<String>) -> String {
    message.into()
}

#[tauri::command]
pub fn app_get_state(state: State<'_, AppState>) -> Result<AppSnapshot, String> {
    state
        .snapshot
        .lock()
        .map(|value| value.clone())
        .map_err(|_| state_error("không thể đọc trạng thái ứng dụng"))
}

#[tauri::command]
pub fn app_set_theme(theme: String, state: State<'_, AppState>) -> Result<AppSnapshot, String> {
    if !["paper", "cocoa", "system"].contains(&theme.as_str()) {
        return Err(state_error("theme không hợp lệ"));
    }
    let mut snapshot = state
        .snapshot
        .lock()
        .map_err(|_| state_error("không thể cập nhật trạng thái"))?;
    snapshot.theme = theme;
    Ok(snapshot.clone())
}

#[tauri::command]
pub fn app_set_typography(
    font_family: String,
    font_scale: u8,
    state: State<'_, AppState>,
) -> Result<AppSnapshot, String> {
    if !(80..=160).contains(&font_scale) {
        return Err(state_error("cỡ chữ phải từ 80% đến 160%"));
    }
    let mut snapshot = state
        .snapshot
        .lock()
        .map_err(|_| state_error("không thể cập nhật trạng thái"))?;
    snapshot.font_family = font_family;
    snapshot.font_scale = font_scale;
    Ok(snapshot.clone())
}

#[tauri::command]
pub fn vault_open(path: String, state: State<'_, AppState>) -> Result<AppSnapshot, String> {
    let vault = Vault::open(&path).map_err(|e| e.to_string())?;
    let mut vault_state = state
        .vault
        .lock()
        .map_err(|_| state_error("không thể mở vault"))?;
    *vault_state = Some(vault);
    let mut snapshot = state
        .snapshot
        .lock()
        .map_err(|_| state_error("không thể cập nhật trạng thái vault"))?;
    snapshot.vault_path = Some(path);
    snapshot.last_error = None;
    Ok(snapshot.clone())
}

#[tauri::command]
pub fn note_read(path: String, state: State<'_, AppState>) -> Result<NotePayload, String> {
    let guard = state
        .vault
        .lock()
        .map_err(|_| state_error("không thể truy cập vault"))?;
    let vault = guard.as_ref().ok_or_else(|| state_error("chưa mở vault"))?;
    let content = vault.read_note(&path).map_err(|e| e.to_string())?;
    Ok(NotePayload { path, content })
}

#[tauri::command]
pub fn note_create(
    path: String,
    content: String,
    state: State<'_, AppState>,
) -> Result<NotePayload, String> {
    let guard = state
        .vault
        .lock()
        .map_err(|_| state_error("không thể truy cập vault"))?;
    let vault = guard.as_ref().ok_or_else(|| state_error("chưa mở vault"))?;
    vault
        .create_note(&path, &content)
        .map_err(|e| e.to_string())?;
    Ok(NotePayload { path, content })
}

#[tauri::command]
pub fn note_update(
    path: String,
    content: String,
    expected_hash: Option<String>,
    state: State<'_, AppState>,
) -> Result<NotePayload, String> {
    let guard = state
        .vault
        .lock()
        .map_err(|_| state_error("không thể truy cập vault"))?;
    let vault = guard.as_ref().ok_or_else(|| state_error("chưa mở vault"))?;
    vault
        .update_note(
            &path,
            &content,
            WriteOptions {
                expected_hash,
                expected_mtime: None,
            },
        )
        .map_err(|e| e.to_string())?;
    Ok(NotePayload { path, content })
}
