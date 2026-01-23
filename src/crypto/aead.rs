use aes_gcm::{
    Aes256Gcm, Nonce,
    aead::{Aead, KeyInit},
};
use rand::RngCore;

use crate::crypto::secret::SecretKey;
use crate::error::{EnvcipherError, Result};

/// Length of AES-256 key in bytes.
pub const KEY_LEN: usize = 32;

/// Length of GCM nonce in bytes.
pub const NONCE_LEN: usize = 12;

// /// Generate cryptographically secure random key.
pub fn generate_key() -> SecretKey {
    let mut key = [0u8; KEY_LEN];
    rand::rng().fill_bytes(&mut key);
    SecretKey::new(key)
}

pub fn aes_encipher(key: &SecretKey, plaintext: &[u8]) -> Result<(Vec<u8>, [u8; NONCE_LEN])> {
    let cipher = Aes256Gcm::new_from_slice(key.as_bytes())
        .map_err(|e| EnvcipherError::Encipherment(e.to_string()))?;

    let nonce_bytes = generate_nonce();
    let nonce = Nonce::from_slice(&nonce_bytes);

    let ciphertext = cipher
        .encrypt(nonce, plaintext)
        .map_err(|e| EnvcipherError::Encipherment(e.to_string()))?;

    Ok((ciphertext, nonce_bytes))
}

/// Collision probability with 96 bits is negligible for our volume.
pub fn generate_nonce() -> [u8; NONCE_LEN] {
    let mut nonce = [0u8; NONCE_LEN];
    rand::rng().fill_bytes(&mut nonce);
    nonce
}

/// Fails if tag validation check fails (tampering detected).
pub fn aes_decipher(
    key: &SecretKey,
    nonce: &[u8; NONCE_LEN],
    ciphertext: &[u8],
) -> Result<Vec<u8>> {
    let cipher = Aes256Gcm::new_from_slice(key.as_bytes())
        .map_err(|e| EnvcipherError::Decipherment(e.to_string()))?;

    let nonce = Nonce::from_slice(nonce);

    cipher
        .decrypt(nonce, ciphertext)
        .map_err(|_| EnvcipherError::Decipherment("authentication failed".to_string()))
}
