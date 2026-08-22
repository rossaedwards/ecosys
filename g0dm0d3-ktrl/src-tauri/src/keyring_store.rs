//! OS keyring for oracle API keys. Never VITE_*, never the renderer.

use keyring::Entry;

const SERVICE: &str = "g0dm0d3-ktrl";

pub fn set_key(hub: &str, secret: &str) -> Result<(), String> {
    let entry = Entry::new(SERVICE, hub).map_err(|e| e.to_string())?;
    entry.set_password(secret).map_err(|e| e.to_string())
}

pub fn get_key(hub: &str) -> Result<String, String> {
    let entry = Entry::new(SERVICE, hub).map_err(|e| e.to_string())?;
    entry.get_password().map_err(|e| e.to_string())
}

pub fn is_bound(hub: &str) -> bool {
    matches!(get_key(hub), Ok(s) if !s.trim().is_empty())
}

pub fn delete_key(hub: &str) -> Result<(), String> {
    let entry = Entry::new(SERVICE, hub).map_err(|e| e.to_string())?;
    match entry.delete_credential() {
        Ok(()) => Ok(()),
        Err(keyring::Error::NoEntry) => Ok(()),
        Err(e) => Err(e.to_string()),
    }
}
