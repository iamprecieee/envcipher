use envcipher::crypto::aead::{aes_decipher, aes_encipher, generate_key, generate_nonce};

#[test]
fn round_trip_encryption() {
    let key = generate_key();
    let plaintext = b"DATABASE_URL=postgres://localhost/mydb";

    let (ciphertext, nonce) = aes_encipher(&key, plaintext).unwrap();
    let decrypted = aes_decipher(&key, &nonce, &ciphertext).unwrap();

    assert_eq!(plaintext.as_slice(), decrypted.as_slice());
}

#[test]
fn wrong_key_fails() {
    let key1 = generate_key();
    let key2 = generate_key();
    let plaintext = b"SECRET=hunter2";

    let (ciphertext, nonce) = aes_encipher(&key1, plaintext).unwrap();
    let result = aes_decipher(&key2, &nonce, &ciphertext);

    assert!(result.is_err());
}

#[test]
fn corrupted_ciphertext_fails() {
    let key = generate_key();
    let plaintext = b"API_KEY=abc123";

    let (mut ciphertext, nonce) = aes_encipher(&key, plaintext).unwrap();

    // Flip a bit in the ciphertext.
    ciphertext[0] ^= 0x01;

    let result = aes_decipher(&key, &nonce, &ciphertext);
    assert!(result.is_err());
}

#[test]
fn wrong_nonce_fails() {
    let key = generate_key();
    let plaintext = b"TOKEN=xyz";

    let (ciphertext, _nonce) = aes_encipher(&key, plaintext).unwrap();
    let wrong_nonce = generate_nonce();

    let result = aes_decipher(&key, &wrong_nonce, &ciphertext);
    assert!(result.is_err());
}

#[test]
fn empty_plaintext_works() {
    let key = generate_key();
    let plaintext = b"";

    let (ciphertext, nonce) = aes_encipher(&key, plaintext).unwrap();
    let decrypted = aes_decipher(&key, &nonce, &ciphertext).unwrap();

    assert!(decrypted.is_empty());
}

#[test]
fn unique_nonces_generated() {
    // Nonces should be different each time.
    let nonce1 = generate_nonce();
    let nonce2 = generate_nonce();

    assert_ne!(nonce1, nonce2);
}
