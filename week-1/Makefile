SHELL := /bin/bash
.PHONY: help example

help:
	@grep -E '^[a-zA-Z_-]+:.*?## .*$$' $(MAKEFILE_LIST) | sort | awk 'BEGIN {FS = ":.*?## "}; {printf "\033[36m%-30s\033[0m %s\n", $$1, $$2}'

all:
	make -C example clean test build
	make -C vector-fruit-salad clean test build
	make -C vecdeque-fruit-salad clean test build
	make -C hashmap-count clean test build
	make -C hashmap-language clean test build

example: ## Build example project
	make -C example clean test build

vector-fruit-salad: ## Build vector-fruit-salad project
	make -C vector-fruit-salad clean test build