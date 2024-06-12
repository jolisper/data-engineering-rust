SHELL := /bin/bash
.PHONY: help all l1-using-rust-to-manage-data-files-and-network-storage l2-dataframes-with-rust-python-and-notebooks

help:
	@grep -E '^[a-zA-Z0-9_-]+:.*?## .*$$' $(MAKEFILE_LIST) | sort | awk 'BEGIN {FS = ":.*?## "}; {printf "\033[36m%-30s\033[0m %s\n", $$1, $$2}'

all: l1-using-rust-to-manage-data-files-and-network-storage l2-dataframes-with-rust-python-and-notebooks ## Build all

l1-using-rust-to-manage-data-files-and-network-storage: ## Build Lesson 1 - Using Rust to Manage Data, Files and Network Storage
	make -C "Lesson 1 - Using Rust to Manage Data, Files and Network Storage" all

l2-dataframes-with-rust-python-and-notebooks: ## Build Lesson 2 - DataFrames with Rust, Python and Notebooks
	make -C "Lesson 2 - DataFrames with Rust, Python and Notebooks" all