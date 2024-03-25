SHELL := /bin/bash
.PHONY: help all mutable-fruit-salad 

help:
	@grep -E '^[a-zA-Z_-]+:.*?## .*$$' $(MAKEFILE_LIST) | sort | awk 'BEGIN {FS = ":.*?## "}; {printf "\033[36m%-30s\033[0m %s\n", $$1, $$2}'

all: mutable-fruit-salad ## Build all projects

mutable-fruit-salad: ## Build mutable-fruit-salad project
	make -C "mutable-fruit-salad" clean test build