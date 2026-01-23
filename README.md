# Envcipher

Secure `.env` file encipherment using OS keychain. Keep secrets enciphered at rest while maintaining developer workflow.

## What It Does

Envcipher enciphers your `.env` files using AES-256-GCM and stores keys in your operating system's secure credential store. This prevents secrets from being committed in plaintext while keeping local development seamless—no separate key files to manage, no complex setup. Decipher on demand, edit securely, or run apps with injected secrets without ever writing plaintext to disk.

## Installation

```bash
cargo install envcipher
```

Or build from source:
```bash
git clone https://github.com/iamprecieee/envcipher
cd envcipher
cargo build --release
```

## Quick Start

```bash
# 1. Initialize in your project
envcipher init

# 2. Add secrets safely
# WARNING: Do NOT append to enciphered file directly!
envcipher edit
# (This opens your $EDITOR, deciphered temporarily, and re-enciphers on save)

# 3. Lock before committing
envcipher lock

# 4. Run without unlocking
envcipher run -- fastapi run dev
```

## Commands Reference

| Command | Description |
|---------|-------------|
| `init` | Generate encipherment key |
| `lock` | Encipher `.env` file in place |
| `unlock` | Decipher `.env` to plaintext |
| `edit` | Securely edit enciphered file (temp file, auto-reencipher) |
| `run -- <cmd>` | Inject deciphered vars into process without disk writes |
| `export-key` | Print base64 key for team sharing |
| `import-key <KEY>` | Import shared key into local keychain |
| `status` | Show encipherment status and key availability |

## Editor Configuration

The `edit` command uses your `$EDITOR` environment variable:

```bash
export EDITOR="code --wait"  # VS Code (--wait is crucial)
export EDITOR="vim"           # Vim
export EDITOR="nano"          # Nano
```

## Team Collaboration

Keys are stored locally by default. To share with teammates:

**Developer A** (exports key):
```bash
envcipher export-key
```

**Developer B** (imports key):
```bash
envcipher import-key <BASE64_KEY>
```

## Security Model

### What It Protects

- **Secrets at rest**: Files enciphered on disk.  
- **Accidental commits**: `.env` enciphered even if committed.  
- **Memory leaks**: Keys zeroized when dropped (no core dump leakage).  
- **Unauthorized local access**: Requires OS login credentials.

### What It Doesn't Protect

- **Running processes**: Secrets are visible in process environment.  
- **Root/admin access**: OS keychain accessible to root.  
- **Key sharing security**: You must use secure channels (key exports are not enciphered).  
- **Side-channel attacks**: Standard software implementation (no constant-time guarantees).

### Threat Model

Envcipher is designed for:
- **Protecting secrets from accidental disclosure** (commits, backups)
- **Local development security** where developers need frequent access but want encipherment at rest
- **Small teams** sharing keys via out-of-band secure channels

Envcipher is **NOT** designed for:
- Production secret management (use Vault, AWS Secrets Manager, etc.)
- Zero-trust environments requiring hardware security modules
- Protection against sophisticated attackers with root access

## FAQ

**Q: Can I edit adding new underlying secrets manually?**
A: **No!** Once enciphered, appending text to the file creates a corrupted state (mixed enciphered/plaintext). Always use `envcipher edit` to modify secrets safely, or `unlock` -> edit -> `lock`.

**Q: Can I commit the enciphered `.env` file?**
A: Yes, but we generally recommend using `.gitignore` and sharing via `export-key`/`import-key` instead. Committing enciphered files is safe only if your team securely shares the key.

**Q: What happens if I lose my key?**  
A: Keys are stored in your OS keychain. If you lose access to your keychain (e.g., fresh OS install), you'll need a teammate to `export-key` and send it to you.

**Q: How do I rotate keys?**  
A: Currently manual: decipher with old key, generate new key via `init` in a fresh directory, re-encipher.

**Q: Does this work in CI/CD?**  
A: The `run` command works, but you'd need to inject the key via `import-key` in your CI setup. For production CI, use native secret management (GitHub Secrets, etc.).

**Q: Can I use this on multiple projects?**  
A: Yes. Each project directory gets its own key (hashed by directory path). Moving a project folder requires re-importing the key.

## Security

- **Encipherment**: AES-256-GCM with 96-bit random nonces
- **Key Generation**: `OsRng` (cryptographically secure)
- **Key Storage**: OS native keyring (Keychain/Credential Manager/Secret Service)
- **Memory Safety**: Keys wrapped in `zeroize::ZeroizeOnDrop`
- **File Format**: `ENVCIPHER:v1:<base64-nonce>:<base64-ciphertext>`

For detailed security analysis, see [SECURITY.md](docs/SECURITY.md).

## Community

- [Contributing](docs/CONTRIBUTING.md)
- [Code of Conduct](docs/CODE_OF_CONDUCT.md)

## License

[MIT](LICENSE)
