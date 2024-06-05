SHELL := /bin/bash
.PHONY: help all csv-writer csv-cookbook

help:
	@grep -E '^[a-zA-Z_-]+:.*?## .*$$' $(MAKEFILE_LIST) | sort | awk 'BEGIN {FS = ":.*?## "}; {printf "\033[36m%-30s\033[0m %s\n", $$1, $$2}'

all: csv-writer csv-cookbook ## Build all projects

csv-writer: ## Build csv-writer project
	make -C "csv-writer" clean test build

csv-cookbook: ## Build csv-cookbook project
	make -C "csv-cookbook" clean test build