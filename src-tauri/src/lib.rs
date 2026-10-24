mod commands;

use commands::{
    app_get_state, app_set_theme, app_set_typography, note_create, note_read, note_update,
    vault_open, AppState,
};

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .manage(AppState::default())
        .invoke_handler(tauri::generate_handler![
            app_get_state,
            app_set_theme,
            app_set_typography,
            vault_open,
            note_read,
            note_create,
            note_update
        ])
        .run(tauri::generate_context!())
        .expect("error while running MilkNote");
}
