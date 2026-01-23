# Security Policy

## Threat Model

Envcipher protects **secrets at rest** on developer machines.

### In Scope

| Threat | Protection |
|--------|------------|
| Accidental git commits | Files encrypted on disk |
| Unauthorized local access | OS keychain requires login |
| Memory leaks | Keys zeroized on drop |

### Out of Scope

- Root/administrator access
- Production environments
- Running process inspection
- Side-channel attacks

---

## Cryptographic Implementation

| Component | Choice | Rationale |
|-----------|--------|-----------|
| Algorithm | AES-256-GCM | AEAD, hardware-accelerated, TLS 1.3 standard |
| Key | 256-bit random | Generated via `OsRng` (CSPRNG) |
| Nonce | 96-bit random | Per-encryption, negligible collision risk |
| Tag | 128-bit | Integrity verification |

---

## Key Storage

| OS | Backend | Access Control |
|----|---------|----------------|
| macOS | Keychain | User login password |
| Windows | Credential Manager | User login |
| Linux | Secret Service | Desktop session |

Keys indexed by `SHA256(project_directory)[0..16]`.

---

## File Format

```
ENVCIPHER:v1:<base64-nonce>:<base64-ciphertext>
```

- Plaintext prefix for clear identification
- Version field for future compatibility
- Base64 encoding for git safety

---

## Known Limitations

1. No automated key rotation
2. Directory-bound keys (moving folders requires re-import)
3. No HSM support
4. Process environment visible to debuggers
5. Temporary plaintext during `edit` (600 permissions)

---

## Vulnerability Disclosure

Email: **emmypresh777@gmail.com**

Do not file public issues for security vulnerabilities.

| Stage | Timeline |
|-------|----------|
| Acknowledgment | 24 hours |
| Assessment | 72 hours |
| Fix | Severity-dependent |

### In Scope

- Cryptographic implementation flaws
- Key leakage beyond documented behavior
- Key storage authentication bypasses

### Out of Scope

- Social engineering
- Physical access attacks
- Root privilege attacks
- Denial of service

---

## Dependencies

| Crate | Purpose |
|-------|---------|
| `aes-gcm` | AES-256-GCM |
| `rand` | CSPRNG |
| `zeroize` | Memory zeroing |
| `keyring` | OS credential store |

Security advisories tracked via `cargo-audit`.
