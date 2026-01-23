use std::{fs, path::Path};

use envcipher::{
    crypto::aead::NONCE_LEN,
    env::parser::{
        find_env_file, format_enciphered_text, hash_directory_path, is_enciphered,
        parse_enciphered_file, parse_env_content,
    },
};
use tempfile::TempDir;

#[test]
fn find_env_in_current_dir() {
    let temp = TempDir::new().unwrap();
    let env_path = temp.path().join(".env");
    fs::write(&env_path, "TEST=value").unwrap();

    let found = find_env_file(temp.path()).unwrap();
    assert_eq!(found, env_path);
}

#[test]
fn find_env_in_parent_dir() {
    let temp = TempDir::new().unwrap();
    let env_path = temp.path().join(".env");
    fs::write(&env_path, "TEST=value").unwrap();

    let subdir = temp.path().join("nested").join("deep");
    fs::create_dir_all(&subdir).unwrap();

    let found = find_env_file(&subdir).unwrap();
    assert_eq!(found, env_path);
}

#[test]
fn stop_at_git_root() {
    let temp = TempDir::new().unwrap();

    // Create .git dir but no .env
    let git_dir = temp.path().join(".git");
    fs::create_dir(&git_dir).unwrap();

    let result = find_env_file(temp.path());
    assert!(result.is_err());
}

#[test]
fn is_enciphered_detection() {
    assert!(is_enciphered("ENVCIPHER:v1:abc:def"));
    assert!(!is_enciphered("DATABASE_URL=postgres://..."));
    assert!(!is_enciphered(""));
}

#[test]
fn format_and_parse_round_trip() {
    let nonce = [1u8; NONCE_LEN];
    let ciphertext = vec![10, 20, 30, 40, 50];

    let formatted = format_enciphered_text(&nonce, &ciphertext);
    let (parsed_nonce, parsed_ciphertext) = parse_enciphered_file(&formatted).unwrap();

    assert_eq!(nonce, parsed_nonce);
    assert_eq!(ciphertext, parsed_ciphertext);
}

#[test]
fn parse_invalid_format_errors() {
    // Missing prefix
    assert!(parse_enciphered_file("abc:def").is_err());

    // Wrong part count
    assert!(parse_enciphered_file("ENVCIPHER:v1:only_one_part").is_err());

    // Invalid base64
    assert!(parse_enciphered_file("ENVCIPHER:v1:!!!:???").is_err());
}

#[test]
fn hash_produces_consistent_output() {
    let path = Path::new("/some/project/path");
    let hash1 = hash_directory_path(path);
    let hash2 = hash_directory_path(path);

    assert_eq!(hash1, hash2);
    assert_eq!(hash1.len(), 16);
}

#[test]
fn hash_differs_for_different_paths() {
    let hash1 = hash_directory_path(Path::new("/path/one"));
    let hash2 = hash_directory_path(Path::new("/path/two"));

    assert_ne!(hash1, hash2);
}

#[test]
fn parse_env_content_handles_quotes_and_comments() {
    let content = r#"
        # Comment
        KEY=VALUE
        QUOTED="hello world"
        SINGLE='foo bar'
        EMPTY=
        "#;

    let vars = parse_env_content(content);
    assert_eq!(vars.len(), 4);
    assert_eq!(vars[0], ("KEY".to_string(), "VALUE".to_string()));
    assert_eq!(vars[1], ("QUOTED".to_string(), "hello world".to_string()));
    assert_eq!(vars[2], ("SINGLE".to_string(), "foo bar".to_string()));
    assert_eq!(vars[3], ("EMPTY".to_string(), "".to_string()));
}
