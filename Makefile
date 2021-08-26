fmt:
	cargo fmt

clippy:
	cargo clippy --all-targets --all-features

test:
	cargo test --all-features --workspace

check: fmt clippy test

bench:
	cargo bench --all-features

clean:
	cargo clean

.PHONY: fmt clippy test check clean
