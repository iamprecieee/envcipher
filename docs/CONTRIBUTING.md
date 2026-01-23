# Contributing to Envcipher

Thank you for your interest in contributing to Envcipher! We welcome contributions from everyone.

## How to Contribute

### Reporting Bugs

If you find a bug, please open an issue with:
- A clear, descriptive title
- Steps to reproduce the issue
- Expected vs actual behavior
- Your environment (OS, Rust version, envcipher version)
- Any relevant logs or error messages

### Suggesting Features

Feature requests are welcome! Please:
- Check existing issues to avoid duplicates
- Clearly describe the use case and benefit
- Consider implementation complexity and security implications

### Submitting Pull Requests

1. **Fork the repository** and create a feature branch
   ```bash
   git checkout -b feature/your-feature-name
   ```

2. **Make your changes**
   - Follow existing code style and conventions
   - Add tests for new functionality
   - Update documentation as needed
   - Ensure all tests pass: `cargo test`
   - Run `cargo clippy` and fix any warnings

3. **Commit your changes**
   - Use clear, descriptive commit messages
   - Reference any related issues

4. **Push and create a PR**
   - Provide a clear description of changes
   - Explain why the change is needed
   - Link to any related issues

### Development Setup

```bash
# Clone your fork
git clone https://github.com/iamprecieee/envcipher
cd envcipher

# Build the project
cargo build

# Run tests
cargo test

# Run integration tests
cargo test --test '*'

# Check code quality
cargo clippy
cargo fmt --check
```

### Code Style

- Follow Rust conventions (use `cargo fmt`)
- Write clear, self-documenting code
- Add comments for complex logic
- Keep functions focused and composable
- Handle errors explicitly (avoid `.unwrap()` in library code)

### Security Contributions

Security is critical for this project. When contributing:
- Be mindful of cryptographic best practices
- Use constant-time operations where appropriate
- Properly handle sensitive data (zeroize secrets)
- Document security assumptions and threat model changes
- For security vulnerabilities, see [SECURITY.md](SECURITY.md)

### Testing

- Write unit tests for new functions
- Add integration tests for new features
- Test edge cases and error conditions
- Verify cross-platform compatibility when possible

### Documentation

- Update README.md for user-facing changes
- Add rustdoc comments for public APIs
- Update SECURITY.md for security-related changes
- Include examples in documentation

## Code of Conduct

This project follows a [Code of Conduct](CODE_OF_CONDUCT.md). By participating, you agree to uphold this code.

## Questions?

Feel free to open an issue for questions or discussion. We're here to help!

## License

By contributing, you agree that your contributions will be licensed under the MIT License.
