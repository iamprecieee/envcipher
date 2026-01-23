# Security Policy

## Threat Model

Envcipher protects **secrets at rest** on developer machines. It's designed for local development workflows where:
- Developers need frequent access to secrets
- Accidental git commits happen
- Team members need to share access

### In Scope

- Encipherment of `.env` files on disk
- Protection against accidental commits
- Memory hygiene (key zeroization)
- Unauthorized local user access

### Out of Scope

- Protection against root/administrator access
- Production secret management
- Protection of running processes (environment variables are visible)
- Side-channel attacks (timing, power analysis)

## Cryptographic Choices

### Encipherment Algorithm: AES-256-GCM

**Why AES-256-GCM?**
- **AEAD** (Authenticated Encipherment with Associated Data): provides both confidentiality and integrity
- **Hardware acceleration**: Most modern CPUs have AES-NI instructions
- **Industry standard**: Used by TLS 1.3, disk encipherment tools
- **Nonce-based**: No need for derived IVs

**Parameters:**
- Key: 256 bits (32 bytes random)
- Nonce: 96 bits (12 bytes random per encipherment)
- Tag: 128 bits (appended to ciphertext automatically)

**Nonce Collision Risk:**
With 96-bit nonces, birthday paradox gives ~2^48 encipherments before 50% collision probability. For local `.env` file usage (dozens-hundreds of encipherments), risk is negligible.

### Key Generation: OsRng

Uses `rand::OsRng` which calls:
- **Linux/macOS**: `getrandom()` syscall or `/dev/urandom`
- **Windows**: `BCryptGenRandom`

These are cryptographically secure random number generators (CSPRNGs).

### Key Derivation: None

Keys are **not** password-derived. Each installation generates a fresh 256-bit random key. This avoids password complexity requirements but requires secure key sharing for teams.

## Key Storage

### Mechanism

Keys are stored in the operating system's native credential manager:

| OS | Backend | Access Control |
|----|---------|----------------|
| macOS | Keychain | Requires user login password |
| Windows | Credential Manager | Requires user login |
| Linux | Secret Service (libsecret) | Desktop session authentication |

### Key Indexing

Keys are indexed by `SHA256(project_directory_path)[0..16]` (16 hex chars).

**Implications:**
- Moving a project folder changes the key ID
- Different users on same machine have isolated keys
- No cross-project key reuse

### Key Export Format

`export-key` prints keys as base64. This is **intentional** - we prioritize simplicity over encipherment-during-transit because:
1. Users sending via 1Password/Signal already have enciphered channels
2. Adding encipherment-at-rest for exports adds complexity without meaningful security gain in typical workflows

**Best Practice:** Send exported keys via existing secure channels (1Password shared vaults, Signal private messages, never Slack/email).

## File Format

```
ENVCIPHER:v1:<base64-nonce>:<base64-ciphertext>
```

**Design choices:**
- **Plaintext prefix**: Clearly identifies enciphered files (no ambiguity)
- **Version field**: Allows future format changes
- **Base64 encoding**: Avoids binary file issues in git

## Memory Safety

### Zeroization

Keys are wrapped in `SecretKey` struct with `#[derive(ZeroizeOnDrop)]`. This ensures:
1. **Deterministic cleanup**: Keys zeroed when variable goes out of scope
2. **Protection against core dumps**: Reduces window where keys exist in memory
3. **No reliance on allocator behavior**: Explicit zeroing before deallocation

### Limitations

- **Compiler optimizations**: Dead store elimination could theoretically skip zeroing (mitigated by `zeroize` crate using volatile writes)
- **Swap/hibernate**: If system swaps memory to disk before zeroing, keys may persist
- **Debug builds**: Debugger-attached processes can read memory pre-zeroing

## Known Limitations

1. **No key rotation**: Changing keys requires manual decipherment/re-encipherment
2. **Directory-bound keys**: Moving project folders requires key re-import
3. **No HSM support**: Keys stored in software keychains, not hardware security modules
4. **Process environment visibility**: `run` command puts secrets in process env (visible to debuggers, `/proc/<pid>/environ`)
5. **Temporary files during edit**: Plaintext exists briefly in `/tmp` with 0600 permissions

## Vulnerability Disclosure

If you discover a security vulnerability, please email:

**emmypresh777@gmail.com**

Please **do not** file public GitHub issues for security vulnerabilities.

### Response Timeline

- **Acknowledgment**: Within 48 hours
- **Initial assessment**: Within 1 week
- **Fix timeline**: Depends on severity (critical: days, high: weeks)

### Scope

In-scope vulnerabilities:
- Cryptographic implementation flaws
- Key leakage beyond documented behavior
- Authentication/authorization bypasses in key storage

Out-of-scope:
- Social engineering attacks
- Physical access attacks (evil maid, etc.)
- Attacks requiring root/administrator privileges
- Denial of service (local tool, no network component)

## Security Best Practices

When using Envcipher:

1. **Lock laptops when away**: OS keychain requires login, but unlocked sessions can access keys
2. **Use `run` in production-like scenarios**: Avoids writing plaintext to disk
3. **Treat `export-key` output as sensitive**: Send only via enciphered channels
4. **Regular backups**: Export keys to secure backup (enciphered password manager) in case of keychain corruption
5. **Audit access**: Use `status` command to verify initialization state
6. **Use `edit` for to add new secrets to locked files**: Avoids writing plaintext to disk

## Cryptographic Dependencies

| Crate | Purpose | Version |
|-------|---------|---------|
| `aes-gcm` | AES-256-GCM implementation | 0.10.3 |
| `rand` | CSPRNG for key/nonce generation | Latest |
| `zeroize` | Memory zeroing | 1.8+ |
| `keyring` | OS credential store access | 3.6+ |

We track security advisories for these dependencies via `cargo-audit`.

---

**Last Updated**: 2026-01-23  
**Version**: 1.0.0
