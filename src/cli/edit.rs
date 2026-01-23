use std::env;
use std::fs;
use std::process::Command;

use colored::Colorize;
use tempfile::NamedTempFile;

use crate::crypto::aead::{aes_decipher, aes_encipher};
use crate::env::parser::{
    find_env_file, format_enciphered_text, hash_directory_path, is_enciphered,
    parse_enciphered_file, read_env_file, write_to_env_file,
};
use crate::error::{EnvcipherError, Result};
use crate::keystore;

pub fn run() -> Result<()> {
    let current_dir = env::current_dir().map_err(EnvcipherError::Io)?;

    let env_path = find_env_file(&current_dir)?;
    let project_dir = env_path.parent().unwrap_or(&current_dir);

    let dir_hash = hash_directory_path(project_dir);
    let key =
        keystore::retrieve_key_from_store(&dir_hash).map_err(|_| EnvcipherError::NotInitialized)?;

    let contents = read_env_file(&env_path)?;
    let mut initial_plaintext = String::new();

    if is_enciphered(&contents) {
        let (nonce, ciphertext) = parse_enciphered_file(&contents)?;
        let plaintext_bytes = aes_decipher(&key, &nonce, &ciphertext)?;
        initial_plaintext = String::from_utf8(plaintext_bytes).map_err(|_| {
            EnvcipherError::Decipherment("deciphered content is not valid UTF-8".to_string())
        })?;
    } else if !contents.is_empty() {
        initial_plaintext = contents;
    }

    // NamedTempFile created with 0600 permissions by default on unix.
    let temp_file = NamedTempFile::new().map_err(EnvcipherError::Io)?;
    fs::write(temp_file.path(), &initial_plaintext).map_err(EnvcipherError::Io)?;

    let editor = get_editor();

    println!("Opening enciphered .env in {}...", editor);

    // Use shell-words to fallback splits (e.g. "code --wait").
    let args = shell_words::split(&editor).map_err(|e| {
        EnvcipherError::EditorFailed(format!("Failed to parse EDITOR command: {}", e))
    })?;

    if args.is_empty() {
        return Err(EnvcipherError::EditorFailed(
            "EDITOR environment variable is empty".to_string(),
        ));
    }

    let status = Command::new(&args[0])
        .args(&args[1..])
        .arg(temp_file.path())
        .status()
        .map_err(|e| EnvcipherError::EditorFailed(format!("failed to launch {}: {}", editor, e)))?;

    if !status.success() {
        return Err(EnvcipherError::EditorFailed(format!(
            "editor {} exited with status {}",
            editor, status
        )));
    }

    let new_plaintext = fs::read_to_string(temp_file.path()).map_err(EnvcipherError::Io)?;

    // Verify it didn't change if the user just quit without saving.
    if new_plaintext == initial_plaintext {
        println!("{}", "No changes made.".yellow());
        return Ok(());
    }

    let (ciphertext, nonce) = aes_encipher(&key, new_plaintext.as_bytes())?;
    let enciphered_content = format_enciphered_text(&nonce, &ciphertext);

    write_to_env_file(&env_path, &enciphered_content)?;

    // Temp file is automatically deleted when `temp_file` goes out of scope here.

    println!("{}", "Changes saved and enciphered!".green().bold());
    Ok(())
}

fn get_editor() -> String {
    if let Ok(editor) = env::var("EDITOR")
        && !editor.is_empty()
    {
        return editor;
    }

    if let Ok(visual) = env::var("VISUAL")
        && !visual.is_empty()
    {
        return visual;
    }

    if is_program_available("vim") {
        return "vim".to_string();
    }

    if is_program_available("nano") {
        return "nano".to_string();
    }

    "vi".to_string()
}

fn is_program_available(program: &str) -> bool {
    Command::new("which")
        .arg(program)
        .output()
        .map(|output| output.status.success())
        .unwrap_or(false)
}
