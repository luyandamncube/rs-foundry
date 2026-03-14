.PHONY: check test fmt clippy run-api

check:
	cargo check

test:
	cargo test

fmt:
	cargo fmt

clippy:
	cargo clippy --all-targets --all-features -- -D warnings

run-api:
	cargo run --bin runner_api
