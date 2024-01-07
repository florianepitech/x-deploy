#
# X-Deploy Makefile
# Generic Makefile for Rust projects
# 2023
#

.PHONY: build-dev \
	build-prod \
	run-dev-api \
	run-prod-api \
	run-dev-cli \
	run-prod-cli \
	run-dev-daemon \
	run-prod-daemon

# Variables

API_NAME = x-deploy-api

CLI_NAME = x-deploy-cli

DAEMON_NAME = x-deploy-daemon

# Targets

all:	build-prod

build-dev:
	cargo build

build-prod:
	cargo build --release

# Api

run-dev-api:
	export RUST_BACKTRACE=1 && cargo watch -x "run --bin $(API_NAME)"

run-prod-api:
	cargo run --release

# Cli

run-dev-cli:
	export RUST_BACKTRACE=1 && cargo run --bin $(CLI_NAME)

run-prod-cli:
	cargo run --release --bin $(CLI_NAME)

# Daemon

run-dev-daemon:
	export RUST_BACKTRACE=1 && cargo watch -x "run --bin $(DAEMON_NAME)"

run-prod-daemon:
	cargo run --release --bin $(DAEMON_NAME)
