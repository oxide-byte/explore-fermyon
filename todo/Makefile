
.PHONY: setup-api build-api setup-ui build-ui

api: setup-api build-api

ui: setup-ui build-ui

setup-api:
ifeq (,$(shell which rustc))
	$(error "Could not found Rust compiler, please install it")
endif
ifeq (,$(shell which cargo))
	$(error "Could not found Cargo, please install it")
endif
	cargo install cargo-lambda
ifeq (,$(shell which sam))
	$(error "Could not found SAM CLI, please install it")
endif

setup-ui:
ifeq (,$(shell which rustc))
	$(error "Could not found Rust compiler, please install it")
endif
ifeq (,$(shell which cargo))
	$(error "Could not found Cargo, please install it")
endif
ifeq (,$(shell which trunk))
	$(error "Could not found Trunk, please install it")
endif
	cargo install --locked wasm-bindgen-cli

build-api:
	cargo build --manifest-path=todo_api/Cargo.toml --target wasm32-wasi --release

build-ui:
	cd todo_ui;trunk build --release