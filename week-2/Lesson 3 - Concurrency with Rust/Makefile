SHELL := /bin/bash
.PHONY: help all concurrency-parallelism

help:
	@grep -E '^[a-zA-Z_-]+:.*?## .*$$' $(MAKEFILE_LIST) | sort | awk 'BEGIN {FS = ":.*?## "}; {printf "\033[36m%-30s\033[0m %s\n", $$1, $$2}'

all: concurrency-parallelism ## Build all projects

concurrency-parallelism: ## Build concurrency-parallelism project
	make -C "concurrency-parallelism" clean test build
