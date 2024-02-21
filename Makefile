STACK_NAME ?= todo-app
FUNCTIONS := get-todos

ARCH := aarch64-unknown-linux-gnu

.PHONY: setup-api build-api deploy-api setup-ui build-ui

api: setup-api build-api deploy-api

ui: setup-ui build-ui upload-s3

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
	cargo lambda build --manifest-path=todo_api/Cargo.toml --release --target $(ARCH)

build-ui:
	cd todo_ui;trunk build --release

deploy-api:
	if [ -f samconfig.toml ]; \
		then sam deploy --stack-name $(STACK_NAME); \
		else sam deploy -g --stack-name $(STACK_NAME); \
	fi

upload-s3:
	aws s3 cp ./todo_ui/dist s3://todo-app-demo1/ --recursive

clean:
	sam delete