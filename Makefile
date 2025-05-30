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

# Run tests
test:
	cargo test

# Run Clippy with all targets/features, fail on warnings
clippy:
	cargo clippy --all-targets --all-features -- -D warnings

# Format code
fmt:
	cargo fmt

# Audit for vulnerable crates
audit:
	cargo install --quiet cargo-audit || true
	cargo audit
