SHELL := /bin/bash
.PHONY: help all mutable-fruit-salad cli-customize-fruit-salad data-race meet-safe-and-unsafe ownership-lifetimes obrm w2l1-lesson-reflections

help:
	@grep -E '^[a-zA-Z_-]+:.*?## .*$$' $(MAKEFILE_LIST) | sort | awk 'BEGIN {FS = ":.*?## "}; {printf "\033[36m%-30s\033[0m %s\n", $$1, $$2}'

all: mutable-fruit-salad cli-customize-fruit-salad data-race meet-safe-and-unsafe ownership-lifetimes obrm w2l1-lesson-reflections ## Build all projects

mutable-fruit-salad: ## Build mutable-fruit-salad project
	make -C "mutable-fruit-salad" clean test build

cli-customize-fruit-salad: ## Build cli-customize-fruit-salad project
	make -C "cli-customize-fruit-salad" clean test build

data-race: ## Build data-race project
	make -C "data-race" clean test build

meet-safe-and-unsafe: ## Build meet-safe-and-unsafe project
	make -C "meet-safe-and-unsafe" clean test build

ownership-lifetimes: ## Build ownership-lifetime project
	make -C "ownership-lifetimes" clean test build

obrm: ## Build obrm project
	make -C "obrm" clean test build

w2l1-lesson-reflections: ## Build w2l1-lesson-reflections project
	make -C "w2l1-lesson-reflections" clean test build