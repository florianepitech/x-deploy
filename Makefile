#
# X-Deploy Makefile
# Generic Makefile for Rust projects
# 2023
#

.PHONY: build-dev-api \
		build-prod \
		run-dev-api \
		run-prod-api

# Variables

API_NAME = x-deploy-api

# Targets

build-dev-api:
	cargo build

build-prod:
	cargo build --release

run-dev-api:
	cargo watch -x "run --bin $(API_NAME)"

run-prod-api:
	cargo run --release
