.PHONY : all test

PYTHON := python3

ARCH := riscv64gc-unknown-none-elf
BUILD_TYPE := release
TARGET := ../kernel/target/$(ARCH)/$(BUILD_TYPE)
OUTPUT := $(TARGET)/output.log
JSON := $(TARGET)/results.json

all: test

test:
	@$(PYTHON) -W ignore check_result/test_runner.py $(OUTPUT) > $(JSON)
	@$(PYTHON) visualize_result.py $(JSON) || exit 0