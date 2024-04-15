SHELL := /bin/bash
.PHONY: help all rust-crypto-hashes rust-software-security decoder-ring w2l2-lesson-reflections

help:
	@grep -E '^[a-zA-Z_-]+:.*?## .*$$' $(MAKEFILE_LIST) | sort | awk 'BEGIN {FS = ":.*?## "}; {printf "\033[36m%-30s\033[0m %s\n", $$1, $$2}'

all: rust-crypto-hashes rust-software-security decoder-ring w2l2-lesson-reflections ## Build all projects

rust-crypto-hashes: ## Build rust-crypto-hashes project
	make -C "rust-crypto_hashes" clean test build

rust-software-security: ## Build rust-software-security project
	make -C "rust-software-security" clean test build

decoder-ring: ## Build decoder-ring project
	make -C "decoder-ring" clean test build

w2l2-lesson-reflections: ## Build w2l2-lesson-reflections project
	make -C "w2l2-lesson-reflections" clean test build