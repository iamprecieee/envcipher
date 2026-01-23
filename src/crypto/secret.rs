use zeroize::{Zeroize, ZeroizeOnDrop};

use crate::crypto::aead::KEY_LEN;

/// A wrapper for the encryption key that zeroizes memory on drop.
#[derive(Debug, Zeroize, ZeroizeOnDrop)]
pub struct SecretKey(pub [u8; KEY_LEN]);

impl SecretKey {
    pub fn new(key: [u8; KEY_LEN]) -> Self {
        Self(key)
    }

    pub fn as_bytes(&self) -> &[u8] {
        &self.0
    }
}
