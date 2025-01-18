# Default development watch command
default: watch-all

# Initialize the database
init-db:
    ./scripts/init_db.sh

# Run tests once
test:
    cargo test

# Run cargo check
check:
    cargo check

# Format code
format:
    cargo fmt

# Security audit
audit:
    cargo audit

# Watch commands
watch-check:
    cargo watch -x check

watch-test:
    cargo watch -x test

# Watch with multiple commands
watch-all:
    cargo watch -x check -x test -x run

# Install development tools
install-tools:
    cargo install cargo-watch
    cargo install cargo-audit

# Clean build artifacts
clean:
    cargo clean

# Show available commands
help:
    @just --list 