use std::path::PathBuf;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum EnvcipherError {
    #[error("Keychain access failed: {0}")]
    KeychainAccess(String),

    #[error("Encipherment failed: {0}")]
    Encipherment(String),

    #[error("Decipherment failed: {0}")]
    Decipherment(String),

    #[error("No .env file found in {0} or parent directory")]
    EnvNotFound(PathBuf),

    #[error(".env file is already enciphered")]
    AlreadyEnciphered,

    #[error(".env file is not enciphered")]
    NotEnciphered,

    #[error("Envcipher not initialized. Run `envcipher init` first")]
    NotInitialized,

    #[error("Envcipher already initialized in this directory")]
    AlreadyInitialized,

    /// File system error.
    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),

    #[error("Editor exited with error: {0}")]
    EditorFailed(String),

    #[error("Invalid enciphered format: {0}")]
    InvalidFormat(String),
}

pub type Result<T> = std::result::Result<T, EnvcipherError>;
