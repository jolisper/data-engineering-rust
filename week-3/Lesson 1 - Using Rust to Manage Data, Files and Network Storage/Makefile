SHELL := /bin/bash
.PHONY: help all csv-writer csv-cookbook parquet w3l1-lesson-reflections

help:
	@grep -E '^[a-zA-Z_-]+:.*?## .*$$' $(MAKEFILE_LIST) | sort | awk 'BEGIN {FS = ":.*?## "}; {printf "\033[36m%-30s\033[0m %s\n", $$1, $$2}'

all: csv-writer csv-cookbook parquet w3l1-lesson-reflections ## Build all projects

csv-writer: ## Build csv-writer project
	make -C "csv-writer" clean test build

csv-cookbook: ## Build csv-cookbook project
	make -C "csv-cookbook" clean test build

parquet: ## Build parquet project
	make -C "parquet" clean test build

w3l1-lesson-reflections: ## Build w3l1-lesson-reflections project
	make -C "w3l1-lesson-reflections" clean test build