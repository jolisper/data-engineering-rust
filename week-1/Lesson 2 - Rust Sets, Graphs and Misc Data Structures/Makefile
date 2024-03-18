SHELL := /bin/bash
.PHONY: help russian-troll-tweets when-use-rust-set rust-iterators neo4j-data-science-lib graph-centrality-ufc hashset-fruit btreeset-fruit binaryheap-fruit pagerank lisbon-shortest-path

help:
	@grep -E '^[a-zA-Z_-]+:.*?## .*$$' $(MAKEFILE_LIST) | sort | awk 'BEGIN {FS = ":.*?## "}; {printf "\033[36m%-30s\033[0m %s\n", $$1, $$2}'

all: russian-troll-tweets when-use-rust-set rust-iterators neo4j-data-science-lib graph-centrality-ufc hashset-fruit btreeset-fruit binaryheap-fruit pagerank lisbon-shortest-path  ## Build all projects

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

hashset-fruit: ## Build hashset-fruit project
	make -C hashset-fruit clean test build

btreeset-fruit: ## Build btreeset-fruit project
	make -C btreeset-fruit clean test build

binaryheap-fruit: ## Build binaryheap-fruit project
	make -C binaryheap-fruit clean test build

pagerank: ## Build page-rank project
	make -C pagerank clean test build

lisbon-shortest-path: ## Build libon-shortest-path project
	make -C lisbon-shortest-path clean test build