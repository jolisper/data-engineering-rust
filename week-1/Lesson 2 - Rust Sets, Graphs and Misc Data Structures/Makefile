SHELL := /bin/bash
.PHONY: help russian-troll-tweets when-use-rust-set rust-iterators neo4j-data-science-lib graph-centrality-ufc

help:
	@grep -E '^[a-zA-Z_-]+:.*?## .*$$' $(MAKEFILE_LIST) | sort | awk 'BEGIN {FS = ":.*?## "}; {printf "\033[36m%-30s\033[0m %s\n", $$1, $$2}'

all: russian-troll-tweets when-use-rust-set rust-iterators neo4j-data-science-lib graph-centrality-ufc ## Build all projects

russian-troll-tweets: ## Build russian-troll-tweets project
	make -C russian-troll-tweets clean test build

when-use-rust-set: ## Build when-use-rust-set project
	make -C when-use-rust-set clean test build

rust-iterators: ## Build rust-iterators project
	make -C rust-iterators clean test build

neo4j-data-science-lib: ## Build neo4j-data-science-lib project
	make -C neo4j-data-science-lib clean test build

graph-centrality-ufc: ## Build graph-centrality-ufc project
	make -C graph-centrality-ufc clean test build