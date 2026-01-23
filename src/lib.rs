pub mod cli;
pub mod crypto;
pub mod env;
pub mod error;
pub mod keystore;

#[cfg(feature = "python")]
use pyo3::prelude::*;

#[cfg(feature = "python")]
#[pymodule]
fn envcipher(_py: Python, m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(load, m)?)?;
    m.add_function(wrap_pyfunction!(main, m)?)?;
    Ok(())
}

/// Entry point for the CLI when installed via pip
#[cfg(feature = "python")]
#[pyfunction]
fn main() -> PyResult<()> {
    let args: Vec<String> = Python::attach(|py| {
        let sys = py.import("sys")?;
        let argv = sys.getattr("argv")?;
        argv.extract()
    })?;

    match crate::cli::execute(args) {
        Ok(_) => Ok(()),
        Err(e) => {
            eprintln!("Error: {}", e);
            std::process::exit(1);
        }
    }
}

#[cfg(feature = "python")]
#[pyfunction]
#[pyo3(signature = (path=None))]
fn load(py: Python, path: Option<String>) -> PyResult<()> {
    use crate::crypto::aead::aes_decipher;
    use crate::env::parser::{
        find_env_file, is_enciphered, parse_enciphered_file, parse_env_content, read_env_file,
    };

    use std::path::PathBuf;

    let env_path = match path {
        Some(p) => PathBuf::from(p),
        None => find_env_file(&std::env::current_dir()?)
            .map_err(|e| PyErr::new::<pyo3::exceptions::PyFileNotFoundError, _>(e.to_string()))?,
    };

    let content = read_env_file(&env_path)
        .map_err(|e| PyErr::new::<pyo3::exceptions::PyIOError, _>(e.to_string()))?;

    let plaintext = if is_enciphered(&content) {
        let (nonce, ciphertext) = parse_enciphered_file(&content)
            .map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(e.to_string()))?;

        let project_dir = env_path.parent().ok_or_else(|| {
            PyErr::new::<pyo3::exceptions::PyValueError, _>("Cannot find parent directory")
        })?;
        let dir_hash = crate::env::parser::hash_directory_path(project_dir);

        let key = crate::keystore::retrieve_key_from_store(&dir_hash).map_err(|e| {
            PyErr::new::<pyo3::exceptions::PyPermissionError, _>(format!("Key access error: {}", e))
        })?;

        let decrypted_bytes = aes_decipher(&key, &nonce, &ciphertext).map_err(|e| {
            PyErr::new::<pyo3::exceptions::PyValueError, _>(format!("Decryption failed: {}", e))
        })?;

        String::from_utf8(decrypted_bytes)
            .map_err(|e| PyErr::new::<pyo3::exceptions::PyUnicodeDecodeError, _>(e.to_string()))?
    } else {
        content
    };

    let os = py.import("os")?;
    let environ = os.getattr("environ")?;

    for (key, value) in parse_env_content(&plaintext) {
        environ.set_item(key, value)?;
    }

    Ok(())
}
