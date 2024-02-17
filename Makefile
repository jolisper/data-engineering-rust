SHELL := /bin/bash
.PHONY: help week-1 week-2 week-3 week-4 all 

help:
	@grep -E '^[a-zA-Z0-9_-]+:.*?## .*$$' $(MAKEFILE_LIST) | sort | awk 'BEGIN {FS = ":.*?## "}; {printf "\033[36m%-30s\033[0m %s\n", $$1, $$2}'

week-1: ## Build week 1
	make -C week-1 all 

week-2: ## Build week 2
	make -C week-2

week-3: ## Build week 3
	make -C week-3

week-4: ## Build week 4
	make -C week-4

all: ## Build all
	make -C week-1 all
	@#make -C week-2
	@#make -C week-3
	@#make -C week-4