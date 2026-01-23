use std::fs;
use std::path::{Path, PathBuf};

use base64::{Engine, engine::general_purpose::STANDARD as BASE64};
use sha2::{Digest, Sha256};

use crate::crypto::aead::NONCE_LEN;
use crate::error::{EnvcipherError, Result};

/// Enciphered file format prefix.
const FORMAT_PREFIX: &str = "ENVCIPHER:v1:";

const ENV_FILENAME: &str = ".env";

/// Stops at project boundary (.git), home dir, or filesystem root.
pub fn find_env_file(start_dir: &Path) -> Result<PathBuf> {
    let home_dir = dirs::home_dir();
    let mut current_dir = start_dir.to_path_buf();

    loop {
        let env_path = current_dir.join(ENV_FILENAME);
        if env_path.exists() {
            return Ok(env_path);
        }

        let git_dir = current_dir.join(".git");
        if git_dir.exists() {
            return Err(EnvcipherError::EnvNotFound(start_dir.to_path_buf()));
        }

        if let Some(ref home) = home_dir
            && &current_dir == home
        {
            return Err(EnvcipherError::EnvNotFound(start_dir.to_path_buf()));
        }

        match current_dir.parent() {
            Some(parent) => current_dir = parent.to_path_buf(),
            None => return Err(EnvcipherError::EnvNotFound(start_dir.to_path_buf())),
        }
    }
}

pub fn hash_directory_path(path: &Path) -> String {
    let path_str = path.to_string_lossy();
    let hash = Sha256::digest(path_str.as_bytes());
    hex::encode(&hash[..8]) // First 8 bytes = 16 hex chars
}

pub fn read_env_file(path: &Path) -> Result<String> {
    fs::read_to_string(path).map_err(EnvcipherError::Io)
}

pub fn write_to_env_file(path: &Path, contents: &str) -> Result<()> {
    fs::write(path, contents).map_err(EnvcipherError::Io)
}

/// Returns true only if:
/// 1. File starts with ENVCIPHER:v1: prefix
/// 2. File contains exactly one non-empty line
/// 3. Line has the expected format (prefix:nonce:ciphertext)
pub fn is_enciphered(contents: &str) -> bool {
    let trimmed = contents.trim();

    if !trimmed.starts_with(FORMAT_PREFIX) {
        return false;
    }

    let non_empty_lines: Vec<&str> = contents
        .lines()
        .filter(|line| !line.trim().is_empty())
        .collect();

    if non_empty_lines.len() != 1 {
        return false;
    }

    let payload = &trimmed[FORMAT_PREFIX.len()..];
    let parts: Vec<&str> = payload.split(':').collect();

    parts.len() == 2
}

pub fn has_corrupted_format(contents: &str) -> bool {
    let has_enciphered_prefix = contents
        .lines()
        .any(|line| line.trim().starts_with(FORMAT_PREFIX));

    if !has_enciphered_prefix {
        return false;
    }

    // If it has the prefix but is_enciphered returns false, it's corrupted.
    !is_enciphered(contents)
}

/// Output: `ENVCIPHER:v1:<base64-nonce>:<base64-ciphertext>\n`.
pub fn format_enciphered_text(nonce: &[u8; NONCE_LEN], ciphertext: &[u8]) -> String {
    let nonce_b64 = BASE64.encode(nonce);
    let ciphertext_b64 = BASE64.encode(ciphertext);
    format!("{}{nonce_b64}:{ciphertext_b64}\n", FORMAT_PREFIX)
}

pub fn parse_enciphered_file(contents: &str) -> Result<([u8; NONCE_LEN], Vec<u8>)> {
    let contents = contents.trim();

    if !contents.starts_with(FORMAT_PREFIX) {
        return Err(EnvcipherError::InvalidFormat(
            "missing ENVCIPHER:v1: prefix".to_string(),
        ));
    }

    let payload = &contents[FORMAT_PREFIX.len()..];
    let parts: Vec<&str> = payload.split(':').collect();

    if parts.len() != 2 {
        return Err(EnvcipherError::InvalidFormat(
            "expected format ENVCIPHER:v1:<nonce>:<ciphertext>".to_string(),
        ));
    }

    let nonce_bytes = BASE64
        .decode(parts[0])
        .map_err(|e| EnvcipherError::InvalidFormat(format!("invalid nonce base64: {}", e)))?;

    if nonce_bytes.len() != NONCE_LEN {
        return Err(EnvcipherError::InvalidFormat(format!(
            "nonce must be {} bytes, got {}",
            NONCE_LEN,
            nonce_bytes.len()
        )));
    }

    let mut nonce = [0u8; NONCE_LEN];
    nonce.copy_from_slice(&nonce_bytes);

    let ciphertext = BASE64
        .decode(parts[1])
        .map_err(|e| EnvcipherError::InvalidFormat(format!("invalid ciphertext base64: {}", e)))?;

    Ok((nonce, ciphertext))
}

pub fn parse_env_content(content: &str) -> Vec<(String, String)> {
    let mut vars = Vec::new();

    for line in content.lines() {
        let line = line.trim();
        if line.is_empty() || line.starts_with('#') {
            continue;
        }

        if let Some((key, value)) = line.split_once('=') {
            let key = key.trim().to_string();
            let mut value = value.trim();

            if ((value.starts_with('"') && value.ends_with('"'))
                || (value.starts_with('\'') && value.ends_with('\'')))
                && value.len() >= 2
            {
                value = &value[1..value.len() - 1];
            }

            vars.push((key, value.to_string()));
        }
    }

    vars
}
