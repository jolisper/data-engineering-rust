SHELL := /bin/bash
.PHONY: help all csv-writer

help:
	@grep -E '^[a-zA-Z_-]+:.*?## .*$$' $(MAKEFILE_LIST) | sort | awk 'BEGIN {FS = ":.*?## "}; {printf "\033[36m%-30s\033[0m %s\n", $$1, $$2}'

all: csv-writer ## Build all projects

csv-writer: ## Build csv-writer project
	make -C "csv-writer" clean test build