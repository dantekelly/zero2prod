# Development-related commands
.PHONY: dev test check format audit watch-check watch-test watch-all install-tools

# Default development watch command
dev: watch-all

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

# Help command
help:
	@echo "Available commands:"
	@echo "  make dev          - Watch and run check, test, and run on changes"
	@echo "  make test         - Run tests once"
	@echo "  make check        - Run cargo check"
	@echo "  make format       - Format code using cargo fmt"
	@echo "  make audit        - Run security audit"
	@echo "  make watch-check  - Watch and run check on changes"
	@echo "  make watch-test   - Watch and run tests on changes"
	@echo "  make watch-all    - Watch and run check, test, and run on changes"
	@echo "  make install-tools- Install required development tools"
	@echo "  make clean        - Clean build artifacts" 