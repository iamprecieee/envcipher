// //! OS credential store integration.
// //! Uses `keyring` (Keychain/Windows Credential Manager/Secret Service).

use keyring::Entry;

use crate::crypto::aead::KEY_LEN;
use crate::crypto::secret::SecretKey;
use crate::error::{EnvcipherError, Result};

// /// Service name used for credential store entries.
const SERVICE_NAME: &str = "envcipher";

pub fn key_exists(directory_hash: &str) -> Result<bool> {
    let entry = create_keyring_entry(directory_hash)?;

    match entry.get_password() {
        Ok(_) => Ok(true),
        Err(keyring::Error::NoEntry) => Ok(false),
        Err(e) => Err(EnvcipherError::KeychainAccess(format!(
            "failed to check key: {}",
            e
        ))),
    }
}

fn create_keyring_entry(directory_hash: &str) -> Result<Entry> {
    Entry::new(SERVICE_NAME, directory_hash)
        .map_err(|e| EnvcipherError::KeychainAccess(format!("failed to create entry: {}", e)))
}

/// Overwrites existing key if present.
pub fn store_key(directory_hash: &str, key: &SecretKey) -> Result<()> {
    let entry = create_keyring_entry(directory_hash)?;

    // Store as base64 to avoid binary encoding issues.
    let key_b64 =
        base64::Engine::encode(&base64::engine::general_purpose::STANDARD, key.as_bytes());

    entry
        .set_password(&key_b64)
        .map_err(|e| EnvcipherError::KeychainAccess(format!("failed to store key: {}", e)))
}

pub fn retrieve_key_from_store(directory_hash: &str) -> Result<SecretKey> {
    let entry = create_keyring_entry(directory_hash)?;

    let key_b64 = entry
        .get_password()
        .map_err(|e| EnvcipherError::KeychainAccess(format!("failed to retrieve key: {}", e)))?;

    let key_bytes = base64::Engine::decode(&base64::engine::general_purpose::STANDARD, &key_b64)
        .map_err(|e| EnvcipherError::KeychainAccess(format!("invalid key format: {}", e)))?;

    if key_bytes.len() != KEY_LEN {
        return Err(EnvcipherError::KeychainAccess(format!(
            "key has wrong length: expected {}, got {}",
            KEY_LEN,
            key_bytes.len()
        )));
    }

    let mut key = [0u8; KEY_LEN];
    key.copy_from_slice(&key_bytes);
    Ok(SecretKey::new(key))
}

pub fn delete_key(directory_hash: &str) -> Result<()> {
    let entry = create_keyring_entry(directory_hash)?;

    match entry.delete_credential() {
        Ok(_) => Ok(()),
        Err(keyring::Error::NoEntry) => Ok(()), // Not an error if key doesn't exist
        Err(e) => Err(EnvcipherError::KeychainAccess(format!(
            "failed to delete key: {}",
            e
        ))),
    }
}
