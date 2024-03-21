SHELL := /bin/bash
.PHONY: help example all fully-connected-graph w1l2-final-reflection 

help:
	@grep -E '^[a-zA-Z_-]+:.*?## .*$$' $(MAKEFILE_LIST) | sort | awk 'BEGIN {FS = ":.*?## "}; {printf "\033[36m%-30s\033[0m %s\n", $$1, $$2}'

all: fully-connected-graph w1l2-final-reflection ## Build all projects

fully-connected-graph: ## Build fully-connected-graph project
	make -C fully-connected-graph clean test build

w1l2-final-reflection: ## Build w1l2-final-reflection project
	make -C w1l2-final-reflection clean test build