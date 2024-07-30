SHELL := /bin/bash

fmt:
	reset
	cargo fmt $(args)

test:
	reset
	if [[ "$(mod)" = "" ]]; then \
	    cargo test $(args) ; \
	else \
 	    cargo test --test $(mod) $(args) ; \
	fi;

clippy:
	reset
	cargo clippy $(args)
