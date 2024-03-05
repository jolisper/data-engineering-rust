SHELL := /bin/bash
.PHONY: help all m1-rust-sequences-and-maps l2-rust-sets-graphs-and-misc-data-structures

help:
	@grep -E '^[a-zA-Z0-9_-]+:.*?## .*$$' $(MAKEFILE_LIST) | sort | awk 'BEGIN {FS = ":.*?## "}; {printf "\033[36m%-30s\033[0m %s\n", $$1, $$2}'

m1-rust-sequences-and-maps: ## Build Module 1 - Rust Sequences and Maps
	make -C "Lesson 1 - Rust Sequences and Maps" all

l2-rust-sets-graphs-and-misc-data-structures: ## Build Lesson 2 - Rust Sets, Graphs and Misc Data Structures
	make -C "Lesson 2 - Rust Sets, Graphs and Misc Data Structures" all

all: m1-rust-sequences-and-maps l2-rust-sets-graphs-and-misc-data-structures ## Build all