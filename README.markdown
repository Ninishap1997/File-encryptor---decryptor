# File Encryptor

A secure and efficient command-line tool for encrypting and decrypting files using the `age` encryption format, built with Rust.

## Features

- **Secure Encryption**: Uses XChaCha20-Poly1305 via the `age` crate for authenticated encryption.
- **Password Protection**: Encrypts and decrypts files with a user-provided passphrase.
- **Streaming Encryption**: Processes large files efficiently with minimal memory usage.
- **Safe Password Input**: Utilizes `rpassword` to securely handle passphrase input.
- **Cross-Platform**: Runs on Windows, macOS, and Linux.

## Installation

1. **Install Rust**:
   - Download and install Rust via [rustup](https://rust-lang.org/tools/install).
   - Verify: `rustc --version` and `cargo --version`.

2. **Clone the Repository**:
   ```bash
   git clone https://github.com/your-username/file-encryptor.git
   cd file-encryptor
   ```

3. **Build the Project**:
   ```bash
   cargo build --release
   ```

## Usage

### Encrypt a File
Encrypt a file with a passphrase:
```bash
cargo run --release -- encrypt <input_file> <output_file>
```
Example:
```bash
cargo run --release -- encrypt document.pdf encrypted.age
```
- Enter a passphrase when prompted.
- Creates `encrypted.age` containing the encrypted data.

### Decrypt a File
Decrypt a file using the same passphrase:
```bash
cargo run --release -- decrypt <input_file> <output_file>
```
Example:
```bash
cargo run --release -- decrypt encrypted.age decrypted.pdf
```
- Enter the same passphrase used for encryption.
- Creates `decrypted.pdf` with the original content.

### Notes
- Ensure file paths with spaces are enclosed in quotes (e.g., `"My Document.pdf"`).
- The output file for encryption typically uses the `.age` extension by convention.
- Decryption requires loading the entire file into memory, so ensure sufficient RAM for large files.

## Dependencies

- [age](https://crates.io/crates/age) (0.10): For encryption and decryption.
- [secrecy](https://crates.io/crates/secrecy) (0.8): For secure passphrase handling.
- [clap](https://crates.io/crates/clap) (4.5): For command-line argument parsing.
- [rpassword](https://crates.io/crates/rpassword) (7.3): For secure password input.
- [anyhow](https://crates.io/crates/anyhow) (1.0): For error handling.

Add to `Cargo.toml`:
```toml
[dependencies]
age = "0.10"
secrecy = "0.8"
clap = { version = "4.5", features = ["derive"] }
rpassword = "7.3"
anyhow = "1.0"
```

## Security Considerations

- **Passphrase Strength**: Use a strong, unique passphrase to ensure security.
- **File Safety**: Store encrypted `.age` files securely, as they contain sensitive data.
- **Memory Usage**: Decryption loads files into memory, which may be a limitation for very large files.
- **No Key Support**: Currently supports passphrase-based encryption only (no public/private keys).

## Contributing

1. Fork the repository.
2. Create a feature branch: `git checkout -b feature-name`.
3. Commit changes: `git commit -m "Add feature"`.
4. Push to the branch: `git push origin feature-name`.
5. Open a pull request.

## License

This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for details.

## Acknowledgments

- Inspired by the `rage` encryption tool and the `age` crate.
- Built with Rust for safety and performance.