SHELL := /bin/bash
.PHONY: help 

help:
	@grep -E '^[a-zA-Z_-]+:.*?## .*$$' $(MAKEFILE_LIST) | sort | awk 'BEGIN {FS = ":.*?## "}; {printf "\033[36m%-30s\033[0m %s\n", $$1, $$2}'

clean: ## Remove all build artifacts
	cargo clean

format: ## Format code
	@rustup component add rustfmt 2> /dev/null
	cargo clippy 

test: ## Run tests
	cargo test

doc: ## Generate documentation
	cargo doc

build: ## Build
	cargo build

bump: ## Bump version
	@echo "Current version: $$(cargo pkgid | grep -o '#.*' | cut -d# -f2)"
	@read -p "Enter new version: " new_version && \
	sed -i "s/version = \".*\"/version = \"$$new_version\"/" Cargo.toml && \
	echo "Updated to new version: $$(cargo pkgid | grep -o '#.*' | cut -d# -f2)"
