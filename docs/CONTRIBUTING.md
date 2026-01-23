# Contributing

## Quick Start

```bash
git clone https://github.com/iamprecieee/envcipher
cd envcipher
cargo build
cargo test
```

## Development

| Command | Purpose |
|---------|---------|
| `cargo build` | Compile |
| `cargo test` | Run tests |
| `cargo test --test '*'` | Integration tests |
| `cargo clippy` | Lint |
| `cargo fmt --check` | Format check |

### Python Bindings

```bash
pip install maturin
maturin develop --features python
```

## Pull Requests

1. Fork and create a feature branch
2. Follow existing code style (`cargo fmt`)
3. Add tests for new functionality
4. Ensure `cargo test` and `cargo clippy` pass
5. Update documentation if needed

## Code Style

- Use `cargo fmt`
- Handle errors explicitly (no `.unwrap()` in library code)
- Add comments for complex logic
- Keep functions focused

## Security

Security is critical. When contributing:

- Use constant-time operations where appropriate
- Properly zeroize secrets
- Document security assumptions
- For vulnerabilities, see [SECURITY.md](SECURITY.md)

## License

Contributions are licensed under MIT.
