use colored::Colorize;
use std::env;
use std::fs;

use crate::crypto::aead::generate_key;
use crate::env::parser::{find_env_file, hash_directory_path};
use crate::error::{EnvcipherError, Result};
use crate::keystore;

pub fn run() -> Result<()> {
    let current_dir = env::current_dir().map_err(EnvcipherError::Io)?;

    let marker_path = current_dir.join(".envcipher.json");
    if marker_path.exists() {
        return Err(EnvcipherError::AlreadyInitialized);
    }

    let env_path = match find_env_file(&current_dir) {
        Ok(path) => {
            println!("Found .env at: {}", path.display());
            path
        }
        Err(_) => {
            let new_env = current_dir.join(".env");
            fs::write(&new_env, "# Environment variables\n").map_err(EnvcipherError::Io)?;
            println!("Created new .env file");
            new_env
        }
    };

    let project_dir = env_path.parent().unwrap_or(&current_dir).to_path_buf();
    let dir_hash = hash_directory_path(&project_dir);

    // Key might exist but marker doesn't (possibly from failed previous init).
    if keystore::key_exists(&dir_hash)? {
        println!(
            "{} Key already exists in credential store. Reusing existing key.",
            "Warning:".yellow()
        );
    } else {
        let key = generate_key();
        keystore::store_key(&dir_hash, &key)?;
        println!("Generated new encipherment key");
    }

    let marker_content = format!(
        r#"{{
        "version": "1",
        "key_id": "{}"
    }}
    "#,
        &dir_hash[..8]
    );
    fs::write(&marker_path, marker_content).map_err(EnvcipherError::Io)?;
    println!("Created .envcipher.json marker");

    println!();
    println!("{}", "Initialization complete!".green().bold());
    println!("Key ID: {}", &dir_hash[..8]);
    println!();
    println!("Next steps:");
    println!("  1. Add your secrets to .env");
    println!("  2. Run {} to encipher", "envcipher lock".cyan());
    println!("  3. Add .env to .gitignore (optional)");

    Ok(())
}
