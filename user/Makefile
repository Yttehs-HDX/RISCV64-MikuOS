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
BINS = $(patsubst $(APP_DIR)/%.rs, $(TARGET_DIR)/%.bin, $(APPS))

OBJCOPY = rust-objcopy
OBJCOPY_FLAGS = --strip-all -O binary

all: objcopy

build:
	@$(PYTHON) build.py

objcopy: build
	@$(foreach elf, $(ELFS), \
		$(OBJCOPY) $(OBJCOPY_FLAGS) $(elf) $(patsubst %, %.bin, $(elf)); \
	)

clean:
	@$(CARGO) clean

%:
	@$(CARGO) $@ $(CARGO_FLAGS)