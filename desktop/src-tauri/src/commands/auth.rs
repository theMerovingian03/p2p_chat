use keyring::Entry;
use tracing::debug;

#[tauri::command]
pub fn save_refresh_token(token: String) -> Result<(), String> {
    debug!("Saving refresh token");
    let entry = Entry::new("p2p_chat", "refresh_token").map_err(|e| e.to_string())?;
    debug!("Created entry!");
    entry.set_password(&token).map_err(|e| e.to_string())
}

#[tauri::command]
pub fn load_refresh_token() -> Result<String, String> {
    debug!("Loading refresh token");
    let entry = Entry::new("p2p_chat", "refresh_token").map_err(|e| e.to_string())?;
    debug!("Created entry!");
    entry.get_password().map_err(|e| e.to_string())
}

#[tauri::command]
pub fn delete_refresh_token() -> Result<(), String> {
    let entry = Entry::new("p2p_chat", "refresh_token").map_err(|e| e.to_string())?;
    entry.delete_credential().map_err(|e| e.to_string())
}
