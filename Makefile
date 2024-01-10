.PHONY: run
run:
	cargo run

.PHONY: fmt
fmt:
	cargo fmt

.PHONY: lint
lint: fmt
	cargo clippy --fix --allow-dirty