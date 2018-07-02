#!/usr/bin/env make

RELEASE_PROFILE := debug

.PHONY: \
		cargo.update \
		cargo.build

cargo.update: Cargo.lock
cargo.build:  target/${RELEASE_PROFILE}

Cargo.lock: \
		Cargo.toml
	cargo update

target/${RELEASE_PROFILE}: \
		Cargo.lock \
		src
