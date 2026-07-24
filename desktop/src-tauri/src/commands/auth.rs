use crate::services::keyring;

#[tauri::command]
pub fn save_refresh_token(token: String) -> Result<(), String> {
    keyring::save_refresh_token(&token).map_err(|e| e.to_string())
}

#[tauri::command]
pub fn load_refresh_token() -> Result<String, String> {
    keyring::load_refresh_token().map_err(|e| e.to_string())
}

#[tauri::command]
pub fn delete_refresh_token() -> Result<(), String> {
    keyring::delete_refresh_token().map_err(|e| e.to_string())
}
