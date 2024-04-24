SHELL := /bin/bash
.PHONY: help all concurrency-parallelism data-races-race-conditions

help:
	@grep -E '^[a-zA-Z_-]+:.*?## .*$$' $(MAKEFILE_LIST) | sort | awk 'BEGIN {FS = ":.*?## "}; {printf "\033[36m%-30s\033[0m %s\n", $$1, $$2}'

all: concurrency-parallelism data-races-race-conditions ## Build all projects

concurrency-parallelism: ## Build concurrency-parallelism project
	make -C "concurrency-parallelism" clean test build

data-races-race-conditions: ## Build data-races-race-conditions project
	make -C "data-races-race-conditions" clean test build