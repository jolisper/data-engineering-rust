SHELL := /bin/bash
.PHONY: help all rust-crypto-hashes 

help:
	@grep -E '^[a-zA-Z_-]+:.*?## .*$$' $(MAKEFILE_LIST) | sort | awk 'BEGIN {FS = ":.*?## "}; {printf "\033[36m%-30s\033[0m %s\n", $$1, $$2}'

all: rust-crypto-hashes ## Build all projects

rust-crypto-hashes: ## Build rust-crypto-hashes project
	make -C "rust-crypto_hashes" clean test build
