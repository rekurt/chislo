# Contributing

Contributions are welcome! Here's how to get started:

## Development

```bash
# Clone the repo
git clone https://github.com/rekurt/chislo.git
cd chislo

# Run tests
cargo test --all-features

# Run clippy
cargo clippy --all-features -- -D warnings

# Format code
cargo fmt

# Build docs
cargo doc --all-features --open
```

## Pull Requests

1. Fork the repository
2. Create a feature branch (`git checkout -b feature/my-feature`)
3. Write tests for your changes
4. Ensure all tests pass (`cargo test --all-features`)
5. Ensure clippy is clean (`cargo clippy --all-features -- -D warnings`)
6. Commit your changes
7. Push and create a pull request

## Reporting Issues

Please report bugs via [GitHub Issues](https://github.com/rekurt/chislo/issues).
