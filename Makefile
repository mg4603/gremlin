.PHONY: run test fmt lint check clean help commit build
BINARY := miniserv

help:
	@echo "make test"
	@echo "make fmt"
	@echo "make run"
	@echo "make lint					# clippy"
	@echo "make check					# fmt then lint then test"
	@echo "make clean"
	@echo "make commit f=<file_name> [m=<message>]		# commit file with optional message"
	@echo "make build					# check then build"

fmt:
	@echo "==> formatting"
	@cargo fmt

test:
	@echo "==> running tests"
	@cargo test -p cli

lint:
	@echo "==> running linter"
	@cargo clippy --all-targets --all-features -- -D warnings

check: fmt lint test
	@echo "==> fmt + lint + test"

clean:
	@echo "==> cleaning up"
	@cargo clean

commit: check
	@git add $(f)
	@if [ -z "$(m)" ]; then \
		git commit; \
	else \
		git commit -m "$(m)"; \
	fi

run: check
	@echo "==> running"
	@cargo run -p cli

build: check
	@echo "==> running build"
	@cargo build 

