#------------------------------------------------------------------------------
# Wasm
#------------------------------------------------------------------------------

.PHONY: lint
lint:
	cargo fmt --all
	cargo clippy --all-targets --all-features

.PHONY: check-deps
check-deps:
	cargo machete

.PHONY: fix
fix:
	cargo fix --allow-staged

.PHONY: build
build:
	cargo build
