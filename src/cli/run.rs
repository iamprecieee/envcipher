use std::env;
use std::process::Command;

use crate::crypto::aead::aes_decipher;
use crate::env::parser::{
    find_env_file, hash_directory_path, is_enciphered, parse_enciphered_file, parse_env_content,
    read_env_file,
};
use crate::error::{EnvcipherError, Result};
use crate::keystore;

pub fn run(args: Vec<String>) -> Result<()> {
    if args.is_empty() {
        return Err(EnvcipherError::Io(std::io::Error::new(
            std::io::ErrorKind::InvalidInput,
            "No command specified",
        )));
    }

    let env_vars = load_env_vars()?;

    let program = &args[0];
    let program_args = &args[1..];

    let mut command = Command::new(program);
    command.args(program_args);
    command.envs(env_vars);

    // Proxy exit status to parent. On Unix, execvp would replace process entirely,
    // but Command::spawn is more portable across platforms.
    let status = command.status().map_err(EnvcipherError::Io)?;

    if let Some(code) = status.code() {
        std::process::exit(code);
    } else {
        std::process::exit(1);
    }
}

fn load_env_vars() -> Result<Vec<(String, String)>> {
    let current_dir = env::current_dir().map_err(EnvcipherError::Io)?;

    let env_path = find_env_file(&current_dir)?;
    let project_dir = env_path.parent().unwrap_or(&current_dir);

    let contents = read_env_file(&env_path)?;
    let plaintext = if is_enciphered(&contents) {
        let (nonce, ciphertext) = parse_enciphered_file(&contents)?;
        let dir_hash = hash_directory_path(project_dir);
        let key = keystore::retrieve_key_from_store(&dir_hash)
            .map_err(|_| EnvcipherError::NotInitialized)?;

        let plaintext_bytes = aes_decipher(&key, &nonce, &ciphertext)?;
        String::from_utf8(plaintext_bytes).map_err(|_| {
            EnvcipherError::Decipherment("deciphered content is not valid UTF-8".to_string())
        })?
    } else {
        contents
    };

    Ok(parse_env_content(&plaintext))
}
