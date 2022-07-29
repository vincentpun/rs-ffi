# Get the Makefile directory.
THIS_MAKEFILE_PATH:=$(word $(words $(MAKEFILE_LIST)),$(MAKEFILE_LIST))
THIS_DIR:=$(shell cd $(dir $(THIS_MAKEFILE_PATH));pwd)

build-all: build-rust
	gcc main.c -o main -lsum_c_calling_rust -L$(THIS_DIR)/../target/release

build-rust:
	cargo build --release

run: build-all
	LD_LIBRARY_PATH=$(THIS_DIR)/../target/release ./main
