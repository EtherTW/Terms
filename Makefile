# Run the main script
.PHONY: run
run:
	cargo run --release --bin terms


# Default target
.DEFAULT_GOAL := run
