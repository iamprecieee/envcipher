use envcipher::{
    crypto::aead::generate_key,
    keystore::retrieve_key_from_store as retrieve_key,
    keystore::{delete_key, key_exists, store_key},
};

fn test_hash() -> String {
    format!("test_{}", rand::random::<u32>())
}

#[test]
fn store_and_retrieve_key() {
    let hash = test_hash();
    let key = generate_key();

    store_key(&hash, &key).unwrap();
    let retrieved = retrieve_key(&hash).unwrap();

    assert_eq!(key.as_bytes(), retrieved.as_bytes());

    // Cleanup
    delete_key(&hash).unwrap();
}

#[test]
fn retrieve_nonexistent_key_fails() {
    let hash = test_hash();

    let result = retrieve_key(&hash);
    assert!(result.is_err());
}

#[test]
fn key_exists_check() {
    let hash = test_hash();
    let key = generate_key();

    assert!(!key_exists(&hash).unwrap());

    store_key(&hash, &key).unwrap();
    assert!(key_exists(&hash).unwrap());

    delete_key(&hash).unwrap();
    assert!(!key_exists(&hash).unwrap());
}

#[test]
fn delete_nonexistent_key_succeeds() {
    let hash = test_hash();

    // Should not error even if key doesn't exist.
    let result = delete_key(&hash);
    assert!(result.is_ok());
}

#[test]
fn overwrite_existing_key() {
    let hash = test_hash();
    let key1 = generate_key();
    let key2 = generate_key();

    store_key(&hash, &key1).unwrap();
    store_key(&hash, &key2).unwrap();

    let retrieved = retrieve_key(&hash).unwrap();
    assert_eq!(key2.as_bytes(), retrieved.as_bytes());

    // Cleanup
    delete_key(&hash).unwrap();
}
