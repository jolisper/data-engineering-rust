SHELL := /bin/bash
.PHONY: help all concurrency-parallelism data-races-race-conditions send-sync atomics distributed-computing-concurrency challenges-opportunities-distributed dining-philosophers w2l3-lesson-reflections rayon-challenge

help:
	@grep -E '^[a-zA-Z_-]+:.*?## .*$$' $(MAKEFILE_LIST) | sort | awk 'BEGIN {FS = ":.*?## "}; {printf "\033[36m%-30s\033[0m %s\n", $$1, $$2}'

all: concurrency-parallelism data-races-race-conditions send-sync atomics distributed-computing-concurrency challenges-opportunities-distributed dining-philosophers w2l3-lesson-reflections rayon-challenge ## Build all projects

concurrency-parallelism: ## Build concurrency-parallelism project
	make -C "concurrency-parallelism" clean test build

data-races-race-conditions: ## Build data-races-race-conditions project
	make -C "data-races-race-conditions" clean test build

send-sync: ## Build send-sync project
	make -C "send-sync" clean test build

atomics: ## Build atomic project
	make -C "atomics" clean test build

distributed-computing-concurrency: ## Build distributed-computing-concurrency project
	make -C "distributed-computing-concurrency" clean test build

challenges-opportunities-distributed: ## Build challenges-opportunities-distributed project
	make -C "challenges-opportunities-distributed" clean test build

dining-philosophers: ## Build dining-philosophers project
	make -C "dining-philosophers" clean test build

w2l3-lesson-reflections: ## Build w2l3-lesson-reflections project
	make -C "w2l3-lesson-reflections" clean test build

rayon-challenge: ## Build rayon-challenge project
	make -C "rayon-challenge" clean test build