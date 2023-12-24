#
# X-Deploy Makefile
# Generic Makefile for Rust projects
# 2023
#

.PHONY: build-dev \
	build-prod \
	run-dev-api \
	run-prod-api

# Variables

API_NAME = x-deploy-api

# Targets

all:	build-prod

build-dev:
	cargo build

build-prod:
	cargo build --release

run-dev-api:
	export RUST_BACKTRACE=1 && cargo watch -x "run --bin $(API_NAME)"

run-prod-api:
	cargo run --release
