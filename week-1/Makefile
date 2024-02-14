SHELL := /bin/bash
.PHONY: help example

help:
	@grep -E '^[a-zA-Z_-]+:.*?## .*$$' $(MAKEFILE_LIST) | sort | awk 'BEGIN {FS = ":.*?## "}; {printf "\033[36m%-30s\033[0m %s\n", $$1, $$2}'

all:
	make -C example clean test build

example: ## Build example project
	make -C example clean test build
