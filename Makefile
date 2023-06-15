# Run the main script
.PHONY: run
run:
	cargo run --release


# Default target
.DEFAULT_GOAL := run
