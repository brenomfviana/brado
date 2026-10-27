.PHONY: fmt test clippy examples

fmt:
	cargo fmt --all

test:
	cargo test $(if $(mod),-p $(mod),--workspace) $(args)

clippy:
	cargo clippy $(if $(mod),-p $(mod),--workspace) $(args)

examples:
	@for dir in examples/*/; do \
		echo "==> $$dir"; \
		(cd "$$dir" && cargo run --quiet) || exit 1; \
	done

