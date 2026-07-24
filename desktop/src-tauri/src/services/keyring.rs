use keyring::Entry;

pub fn save_refresh_token(token: &str) -> Result<(), keyring::Error> {
    let entry = Entry::new("p2p_chat", "refresh_token")?;
    entry.set_password(token)
}

pub fn load_refresh_token() -> Result<String, keyring::Error> {
    let entry = Entry::new("p2p-chat", "refresh_token")?;
    entry.get_password()
}

pub fn delete_refresh_token() -> Result<(), keyring::Error> {
    let entry = Entry::new("p2p_chat", "refresh_token")?;
    entry.delete_credential()
}
