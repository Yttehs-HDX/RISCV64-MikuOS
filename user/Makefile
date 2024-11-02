.PHONY : all

CARGO = cargo
PYTHON = python3
BUILD_TYPE = release
CARGO_FLAGS = --$(BUILD_TYPE)

ARCH = riscv64gc-unknown-none-elf

APP_DIR = src/bin
TARGET_DIR = target/$(ARCH)/$(BUILD_TYPE)

APPS = $(wildcard $(APP_DIR)/*.rs)
ELFS = $(patsubst $(APP_DIR)/%.rs, $(TARGET_DIR)/%, $(APPS))

all: build

build:
	@$(CARGO) build $(CARGO_FLAGS)

clean:
	@$(CARGO) clean

%:
	@$(CARGO) $@ $(CARGO_FLAGS)