use std::env;

use colored::Colorize;

use crate::crypto::aead::aes_encipher;
use crate::env::parser::{
    find_env_file, format_enciphered_text, has_corrupted_format, hash_directory_path,
    is_enciphered, read_env_file, write_to_env_file,
};
use crate::error::{EnvcipherError, Result};
use crate::keystore;

pub fn run() -> Result<()> {
    let current_dir = env::current_dir().map_err(EnvcipherError::Io)?;

    let env_path = find_env_file(&current_dir)?;
    let project_dir = env_path.parent().unwrap_or(&current_dir);

    let contents = read_env_file(&env_path)?;

    if is_enciphered(&contents) {
        return Err(EnvcipherError::AlreadyEnciphered);
    }

    if has_corrupted_format(&contents) {
        println!(
            "{} File contains both enciphered and plaintext content.",
            "Warning:".yellow()
        );
        println!(
            "This will create nested encipherment. Run {} first to recover the data.",
            "envcipher unlock".cyan()
        );
        println!();
    }

    let dir_hash = hash_directory_path(project_dir);
    let key =
        keystore::retrieve_key_from_store(&dir_hash).map_err(|_| EnvcipherError::NotInitialized)?;

    let (ciphertext, nonce) = aes_encipher(&key, contents.as_bytes())?;

    let enciphered_content = format_enciphered_text(&nonce, &ciphertext);
    write_to_env_file(&env_path, &enciphered_content)?;

    println!("{}", "Locked!".green().bold());
    println!("File: {}", env_path.display());
    println!();
    println!(
        "Your .env is now enciphered. Run {} to decipher.",
        "envcipher unlock".cyan()
    );

    Ok(())
}
