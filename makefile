PROJECT_ROOT := $(shell pwd)
CARGO := cargo

.PHONY: build build-% release

build-%:
	@echo "Building $*"
	@$(CARGO) build --manifest-path $(PROJECT_ROOT)/crates/$*/Cargo.toml

build:
	@$(CARGO) build

release:
	@$(CARGO) build --release
