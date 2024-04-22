SHELL := /bin/bash
.PHONY: help all l1-rust-safety-and-security-features l2-security-programming-with-rust l3-concurrency-with-rust

help:
	@grep -E '^[a-zA-Z0-9_-]+:.*?## .*$$' $(MAKEFILE_LIST) | sort | awk 'BEGIN {FS = ":.*?## "}; {printf "\033[36m%-30s\033[0m %s\n", $$1, $$2}'

l1-rust-safety-and-security-features: ## Build Lesson 1 - Rust Safety and Security Features
	make -C "Lesson 1 - Rust Safety and Security Features" all

l2-security-programming-with-rust: ## Build Lesson 2 - Security Programming with Rust
	make -C "Lesson 2 - Security Programming with Rust" all

l3-concurrency-with-rust: ## Build Lesson 3 - Concurrency in Rust
	make -C "Lesson 3 - Concurrency with Rust" all

all: l1-rust-safety-and-security-features l2-security-programming-with-rust l3-concurrency-with-rust ## Build all