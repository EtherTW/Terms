# Run the main script
.PHONY: run
run:
	cargo run --release --bin terms

clippy:
	cargo clippy

# Default target
.DEFAULT_GOAL := run
