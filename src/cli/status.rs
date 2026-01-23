use std::env;
use std::fs;

use colored::Colorize;

use crate::env::parser::{find_env_file, hash_directory_path, is_enciphered, read_env_file};
use crate::error::{EnvcipherError, Result};
use crate::keystore;

pub fn run() -> Result<()> {
    let current_dir = env::current_dir().map_err(EnvcipherError::Io)?;

    let marker_path = current_dir.join(".envcipher.json");
    let initialized = marker_path.exists();

    let env_result = find_env_file(&current_dir);

    println!("{}", "envcipher status".bold());
    println!("────────────────────────────────────────");
    println!("Directory:   {}", current_dir.display());

    if initialized {
        println!("Initialized: {}", "Yes".green());
    } else {
        println!("Initialized: {}", "No".yellow());
        println!("Run {} to initialize.", "envcipher init".cyan());
        return Ok(());
    }

    match env_result {
        Ok(env_path) => {
            let project_dir = env_path.parent().unwrap_or(&current_dir);

            match read_env_file(&env_path) {
                Ok(contents) => {
                    if is_enciphered(&contents) {
                        println!("Status:      {}", "Locked (enciphered)".green());
                    } else {
                        println!("Status:      {}", "Unlocked (EXPOSED)".red().bold());
                    }

                    if let Ok(metadata) = fs::metadata(&env_path)
                        && let Ok(modified) = metadata.modified()
                    {
                        println!("Modified:    {:?}", modified);
                    }

                    let dir_hash = hash_directory_path(project_dir);
                    match keystore::key_exists(&dir_hash) {
                        Ok(true) => {
                            println!("Key ID:      {}", &dir_hash[..8]);
                        }
                        Ok(false) => {
                            println!("Key:         {}", "Not found in credential store".red());
                        }
                        Err(_) => {
                            println!("Key:         {}", "Error checking credential store".red());
                        }
                    }
                }
                Err(e) => {
                    println!("Env file:    Error reading: {}", e);
                }
            }
        }
        Err(_) => {
            println!("Env file:    {}", "Not found".yellow());
        }
    }

    Ok(())
}
