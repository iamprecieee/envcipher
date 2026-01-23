# Envcipher

[![Crates.io](https://img.shields.io/crates/v/envcipher.svg)](https://crates.io/crates/envcipher)
[![PyPI](https://img.shields.io/pypi/v/envcipher.svg)](https://pypi.org/project/envcipher/)
[![License: MIT](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)

Encrypt `.env` files using AES-256-GCM with keys stored in your OS keychain. Decrypt on demand for local development without managing separate key files.

---

## Installation

<details open>
<summary><strong>Python</strong></summary>

```bash
pip install envcipher
```

Provides both the CLI and Python library.

</details>

<details>
<summary><strong>Rust</strong></summary>

```bash
cargo install envcipher
```

CLI only.

</details>

<details>
<summary><strong>From Source</strong></summary>

```bash
git clone https://github.com/iamprecieee/envcipher
cd envcipher
cargo build --release
```

</details>

---

## Usage

### CLI

```bash
envcipher init          # Generate key, store in OS keychain
envcipher edit          # Decrypt -> edit -> re-encrypt
envcipher lock          # Encrypt .env in place
envcipher unlock        # Decrypt .env to plaintext
envcipher run -- <cmd>  # Run command with decrypted env vars
envcipher status        # Show encryption status
```

<details>
<summary><strong>Python Library</strong></summary>

```python
import envcipher
import os

# Load encrypted .env into os.environ
envcipher.load()

# Access secrets
api_key = os.getenv("API_KEY")
```

Custom path:

```python
envcipher.load(path="/path/to/.env")
```

Works with both encrypted and plaintext files.

</details>

---

## Team Sharing

```bash
# Export key
envcipher export-key
# Output: qQWntX6r7eANxsyKHbkJtuXtzW0Hy5zjJGvDSxMKM9I=

# Import on another machine
envcipher import-key qQWntX6r7eANxsyKHbkJtuXtzW0Hy5zjJGvDSxMKM9I=
```

Share keys through secure channels only.

---

## Security

| Component | Implementation |
|-----------|----------------|
| Encryption | AES-256-GCM, 96-bit random nonces |
| Key Storage | OS keychain (Keychain / Credential Manager / Secret Service) |
| Memory | Keys zeroized on drop |
| Format | `ENVCIPHER:v1:<nonce>:<ciphertext>` |

**Designed for:** Protecting secrets from accidental commits, local development encryption at rest, small team key sharing.

**Not designed for:** Production secret management, zero-trust environments, HSM requirements.

---

## FAQ

<details>
<summary>Can I manually edit the encrypted file?</summary>

No. Use `envcipher edit` or the unlock-edit-lock workflow. Manual edits corrupt the format.

</details>

<details>
<summary>Can I commit the encrypted .env file?</summary>

Yes, but we recommend using `.gitignore` and sharing via `export-key`/`import-key` instead. Committing encrypted files is safe only if your team securely shares the key.

</details>

<details>
<summary>What if I lose my key?</summary>

Keys are stored in your OS keychain. If you lose access (e.g., fresh OS install), get a teammate to run `export-key`.

</details>

<details>
<summary>How do I rotate keys?</summary>

Currently manual: decrypt with old key, run `init` in a fresh directory to generate new key, re-encrypt.

</details>

<details>
<summary>Does it work in CI/CD?</summary>

Not recommended. Envcipher is designed for local development. CI runners have ephemeral keychains, and storing the key as a CI secret defeats the purpose. Use native secret management instead (GitHub Secrets, AWS Secrets Manager, etc.).

</details>

<details>
<summary>Can I use this on multiple projects?</summary>

Yes. Each project directory gets its own key (hashed by directory path). Moving a project folder requires re-importing the key.

</details>

---

## License

[MIT](LICENSE)

---

[Contributing](docs/CONTRIBUTING.md) | [Code of Conduct](docs/CODE_OF_CONDUCT.md) | [Security](docs/SECURITY.md)
