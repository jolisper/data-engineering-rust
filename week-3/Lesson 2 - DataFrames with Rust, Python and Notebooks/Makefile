SHELL := /bin/bash
.PHONY: help all rust-jupyter-notebook 

help:
	@grep -E '^[a-zA-Z_-]+:.*?## .*$$' $(MAKEFILE_LIST) | sort | awk 'BEGIN {FS = ":.*?## "}; {printf "\033[36m%-30s\033[0m %s\n", $$1, $$2}'

all: rust-jupyter-notebook ## Build all projects

rust-jupyter-notebook: ## Build rust-jupyter-notebook project
	make -C "rust-jupyter-notebook" clean test build