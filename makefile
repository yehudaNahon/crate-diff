#!/usr/bin/make -f

.DEFAULT_GOAL := help

.PHONY: help init sync

MIRROR_NAME = crates-mirror
PACK_FOLDER_NAME = crates-pack
LAST_COMMIT = 86cc3264b7ed6c76582563c029d613844ce2ef46

init: ## Setup the enviroment and install needed programms
	cargo install panamax
	panamax init $(MIRROR_NAME)
	ln -sf ../mirror.toml $(MIRROR_NAME)/mirror.toml

sync: ## Start syncing crates.io
	panamax sync $(MIRROR_NAME)
	
pack:
	cargo run -- $(MIRROR_NAME)/crates.io-index $(MIRROR_NAME)/crates $(PACK_FOLDER_NAME) $(LAST_COMMIT)

help: ## Display this help text
	@grep -E '^[a-zA-Z_-]+:.*?## .*$$' $(MAKEFILE_LIST) | sort | awk 'BEGIN {FS = ":.*?## "}; {printf "\033[36m%-30s\033[0m %s\n", $$1, $$2}'
