SHELL := /bin/bash
.PHONY: help all mutable-fruit-salad cli-customize-fruit-salad data-race

help:
	@grep -E '^[a-zA-Z_-]+:.*?## .*$$' $(MAKEFILE_LIST) | sort | awk 'BEGIN {FS = ":.*?## "}; {printf "\033[36m%-30s\033[0m %s\n", $$1, $$2}'

all: mutable-fruit-salad cli-customize-fruit-salad data-race ## Build all projects

mutable-fruit-salad: ## Build mutable-fruit-salad project
	make -C "mutable-fruit-salad" clean test build

cli-customize-fruit-salad: ## Build cli-customize-fruit-salad project
	make -C "cli-customize-fruit-salad" clean test build

data-race: ## Build data-race project
	make -C "data-race" clean test build