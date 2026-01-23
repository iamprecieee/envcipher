use std::env;

use colored::Colorize;

use crate::crypto::aead::aes_decipher;
use crate::env::parser::{
    find_env_file, has_corrupted_format, hash_directory_path, is_enciphered, parse_enciphered_file,
    read_env_file, write_to_env_file,
};
use crate::error::{EnvcipherError, Result};
use crate::keystore;

pub fn run() -> Result<()> {
    let current_dir = env::current_dir().map_err(EnvcipherError::Io)?;

    let env_path = find_env_file(&current_dir)?;
    let project_dir = env_path.parent().unwrap_or(&current_dir);

    let contents = read_env_file(&env_path)?;

    if !is_enciphered(&contents) {
        return Err(EnvcipherError::NotEnciphered);
    }

    let dir_hash = hash_directory_path(project_dir);
    let key =
        keystore::retrieve_key_from_store(&dir_hash).map_err(|_| EnvcipherError::NotInitialized)?;

    // Recursively decrypt in case of nested encipherment (from mixed content being locked).
    let mut plaintext_str = contents;
    let mut decipherment_count = 0;
    const MAX_DECIPHERMENT_ATTEMPTS: usize = 10; // Prevent infinite loops

    while decipherment_count < MAX_DECIPHERMENT_ATTEMPTS {
        if is_enciphered(&plaintext_str) {
            let (nonce, ciphertext) = parse_enciphered_file(&plaintext_str)?;
            let plaintext = aes_decipher(&key, &nonce, &ciphertext)?;
            plaintext_str = String::from_utf8(plaintext).map_err(|_| {
                EnvcipherError::Decipherment("deciphered content is not valid UTF-8".to_string())
            })?;
            decipherment_count += 1;
        } else if has_corrupted_format(&plaintext_str) {
            // Mixed content.
            let mut deciphered_lines = Vec::new();
            let mut found_enciphered = false;

            for line in plaintext_str.lines() {
                let line = line.trim();
                if line.starts_with("ENVCIPHER:v1:") {
                    // Try to decipher this embedded line.
                    match parse_enciphered_file(line) {
                        Ok((nonce, ciphertext)) => match aes_decipher(&key, &nonce, &ciphertext) {
                            Ok(plaintext) => {
                                let deciphered_line =
                                    String::from_utf8(plaintext).map_err(|_| {
                                        EnvcipherError::Decipherment(
                                            "deciphered content is not valid UTF-8".to_string(),
                                        )
                                    })?;
                                deciphered_lines.push(deciphered_line);
                                found_enciphered = true;
                            }
                            Err(_) => deciphered_lines.push(line.to_string()),
                        },
                        Err(_) => deciphered_lines.push(line.to_string()),
                    }
                } else if !line.is_empty() {
                    deciphered_lines.push(line.to_string());
                }
            }

            if found_enciphered {
                plaintext_str = deciphered_lines.join("\n") + "\n";
                decipherment_count += 1;
            } else {
                break; // No more enciphered content found
            }
        } else {
            break; // Fully deciphered
        }
    }

    if decipherment_count > 1 {
        println!(
            "{} Detected nested encipherment ({} layers). File has been recovered.",
            "Warning:".yellow(),
            decipherment_count
        );
    }

    write_to_env_file(&env_path, &plaintext_str)?;

    println!("{}", "Unlocked!".green().bold());
    println!("File: {}", env_path.display());
    println!();
    println!(
        "{} Plaintext secrets are exposed on disk. Run {} when done.",
        "Warning:".yellow(),
        "envcipher lock".cyan()
    );

    Ok(())
}
