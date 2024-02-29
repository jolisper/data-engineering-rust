SHELL := /bin/bash
.PHONY: help example all vector-fruit-salad vecdeque-fruit-salad linked-list-fruit-salad hashmap-count hashmap-language cli-salad

help:
	@grep -E '^[a-zA-Z_-]+:.*?## .*$$' $(MAKEFILE_LIST) | sort | awk 'BEGIN {FS = ":.*?## "}; {printf "\033[36m%-30s\033[0m %s\n", $$1, $$2}'

all: example vector-fruit-salad vecdeque-fruit-salad linked-list-fruit-salad hashmap-count hashmap-language cli-salad ## Build all projects

example: ## Build example project
	make -C example clean test build

vector-fruit-salad: ## Build vector-fruit-salad project
	make -C vector-fruit-salad clean test build

vecdeque-fruit-salad: ## Build vecdeque-fruit-salad project
	make -C vecdeque-fruit-salad clean test build

linked-list-fruit-salad: ## Build linked-list-fruit-salad project
	make -C linked-list-fruit-salad clean test build

hashmap-count: ## Build hashmap-count project
	make -C hashmap-count clean test build

hashmap-language: ## Build hashmap-language project
	make -C hashmap-language clean test build

cli-salad: ## Build cli-salad project
	make -C cli-salad clean test build

rust-collections-docs: ## Build rust-collections-docs
	make -C rust-collections-docs clean test build