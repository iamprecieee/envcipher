use std::env;

use base64::{Engine, engine::general_purpose::STANDARD as BASE64};
use colored::Colorize;

use crate::crypto::aead::KEY_LEN;
use crate::crypto::secret::SecretKey;
use crate::env::parser::{find_env_file, hash_directory_path};
use crate::error::{EnvcipherError, Result};
use crate::keystore;

pub fn export() -> Result<()> {
    let current_dir = env::current_dir().map_err(EnvcipherError::Io)?;

    // Identify project via .env location.
    let env_path = find_env_file(&current_dir)?;
    let project_dir = env_path.parent().unwrap_or(&current_dir);
    let dir_hash = hash_directory_path(project_dir);

    let key =
        keystore::retrieve_key_from_store(&dir_hash).map_err(|_| EnvcipherError::NotInitialized)?;

    let key_b64 = BASE64.encode(key.as_bytes());

    println!("{}", "Envcipher Key Export".bold());
    println!("{}", "───────────────────".dimmed());
    println!("Share this key securely with your team.");
    println!("They should run: `envcipher import-key <KEY>`");
    println!();
    println!("{}", key_b64.green().bold());

    Ok(())
}

pub fn import(key_str: &str) -> Result<()> {
    let current_dir = env::current_dir().map_err(EnvcipherError::Io)?;

    let key_bytes = BASE64
        .decode(key_str)
        .map_err(|e| EnvcipherError::InvalidFormat(format!("Invalid key format: {}", e)))?;

    if key_bytes.len() != KEY_LEN {
        return Err(EnvcipherError::InvalidFormat(format!(
            "Key must be {} bytes, got {}",
            KEY_LEN,
            key_bytes.len()
        )));
    }

    let mut key = SecretKey::new([0u8; KEY_LEN]);
    key.0.copy_from_slice(&key_bytes);

    // import-key works even without .env present (e.g., fresh clone scenario).
    let project_dir = match find_env_file(&current_dir) {
        Ok(path) => path.parent().unwrap_or(&current_dir).to_path_buf(),
        Err(_) => current_dir,
    };

    let dir_hash = hash_directory_path(&project_dir);

    keystore::store_key(&dir_hash, &key)?;

    println!("{}", "Key imported successfully!".green().bold());
    println!("Project: {}", project_dir.display());

    Ok(())
}
