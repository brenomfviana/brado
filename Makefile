.PHONY: fmt test clippy

fmt:
	cargo fmt --all

test:
	cargo test $(if $(mod),-p $(mod),--workspace) $(args)

clippy:
	cargo clippy $(if $(mod),-p $(mod),--workspace) $(args)
