SHELL := /bin/bash
.PHONY: help russian-troll-tweets

help:
	@grep -E '^[a-zA-Z_-]+:.*?## .*$$' $(MAKEFILE_LIST) | sort | awk 'BEGIN {FS = ":.*?## "}; {printf "\033[36m%-30s\033[0m %s\n", $$1, $$2}'

all: russian-troll-tweets ## Build all projects

russian-troll-tweets: ## Build russian-troll-tweets project
	make -C russian-troll-tweets clean test build