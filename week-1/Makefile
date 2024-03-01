SHELL := /bin/bash
.PHONY: help all m1-rust-sequences-and-maps 

help:
	@grep -E '^[a-zA-Z0-9_-]+:.*?## .*$$' $(MAKEFILE_LIST) | sort | awk 'BEGIN {FS = ":.*?## "}; {printf "\033[36m%-30s\033[0m %s\n", $$1, $$2}'

m1-rust-sequences-and-maps: ## Build Module 1 - Rust Sequences and Maps
	make -C "Module 1 - Rust Sequences and Maps" all

all: m1-rust-sequences-and-maps ## Build all