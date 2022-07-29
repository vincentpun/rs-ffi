# Get the Makefile directory.
THIS_MAKEFILE_PATH:=$(word $(words $(MAKEFILE_LIST)),$(MAKEFILE_LIST))
THIS_DIR:=$(shell cd $(dir $(THIS_MAKEFILE_PATH));pwd)
BUILD_DIR:=$(THIS_DIR)/../build

build-lib:
	gcc -c -fPIC lib.c -o $(BUILD_DIR)/lib.o
	gcc $(BUILD_DIR)/lib.o -shared -o $(BUILD_DIR)/libsum_bindgen.so
