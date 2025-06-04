@echo "  test                    - Run tests"
	@echo "  test-nocapture          - Run tests with output visible".PHONY: build build-release build-linux-musl-debug build-linux-musl-release clippy fmt audit clean check install help

# Default target
all: build

# Build Debug
build:
	cargo build

# Build Release
build-release:
	cargo build --release

# Build Debug with MUSL (static binary)
build-linux-musl-debug:
	cargo build --target x86_64-unknown-linux-musl

# Build Release with MUSL
build-linux-musl-release:
	cargo build --release --target x86_64-unknown-linux-musl



# Run Clippy with all targets/features, fail on warnings
clippy:
	cargo clippy --all-targets --all-features -- -D warnings

# Format code
fmt:
	cargo fmt

# Check formatting without making changes
fmt-check:
	cargo fmt -- --check

# Audit for vulnerable crates
audit:
	cargo install --quiet cargo-audit || true
	cargo audit

# Clean build artifacts
clean:
	cargo clean

# Run all checks (format, clippy, build)
check: fmt-check clippy build

# Install the binary to ~/.cargo/bin
install:
	cargo install --path .

# Install the binary to ~/.cargo/bin (release mode)
install-release:
	cargo install --path . --release

# Show help
help:
	@echo "Available targets:"
	@echo "  build                    - Build debug version"
	@echo "  build-release           - Build release version"
	@echo "  build-linux-musl-debug  - Build debug static binary (Linux MUSL)"
	@echo "  build-linux-musl-release- Build release static binary (Linux MUSL)"
	@echo "  test                    - Run tests"
	@echo "  test-nocapture          - Run tests with output visible"
	@echo "  clippy                  - Run clippy linter"
	@echo "  fmt                     - Format code"
	@echo "  fmt-check               - Check code formatting"
	@echo "  audit                   - Run security audit"
	@echo "  clean                   - Clean build artifacts"
	@echo "  check                   - Run all checks (fmt, clippy, build)"
	@echo "  install                 - Install binary (debug)"
	@echo "  install-release         - Install binary (release)"
	@echo "  help                    - Show this help message"