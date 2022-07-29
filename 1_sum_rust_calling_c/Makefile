# Get the Makefile directory.
THIS_MAKEFILE_PATH:=$(word $(words $(MAKEFILE_LIST)),$(MAKEFILE_LIST))
THIS_DIR:=$(shell cd $(dir $(THIS_MAKEFILE_PATH));pwd)

run: build
	LD_LIBRARY_PATH=$(THIS_DIR)/build target/release/sum_rust_calling_c

build: $(shell find rs-src -iname "*.rs" -type f)
	$(MAKE) -C c-src
	cargo build --release
