.PHONY : all build clean clippy fmt fs

CARGO = cargo
BUILD_TYPE = release
CARGO_FLAGS = --$(BUILD_TYPE)

ARCH = riscv64gc-unknown-none-elf

APP_DIR = src/bin
TARGET_DIR = target/$(ARCH)/$(BUILD_TYPE)

APPS = $(wildcard $(APP_DIR)/*.rs)
ELFS = $(patsubst $(APP_DIR)/%.rs, $(TARGET_DIR)/%, $(APPS))

FS_IMG = sdcard.img
FS_PATH = $(TARGET_DIR)/sdcard.img
FS_MOUNT = $(TARGET_DIR)/mnt

all: fs

build:
	@$(CARGO) build $(CARGO_FLAGS)

fs: build
	@dd if=/dev/zero of=$(FS_PATH) bs=1M count=20
	@mkfs.fat $(FS_PATH)
	@mkdir -p $(FS_MOUNT)
	@sudo mount -o loop $(FS_PATH) $(TARGET_DIR)/mnt
	@for elf in $(ELFS); do \
		sudo cp $$elf $(FS_MOUNT); \
	done
	@sudo umount $(FS_MOUNT)

clean:
	@$(CARGO) clean

clippy:
	@$(CARGO) clippy

fmt:
	@$(CARGO) fmt

%:
	@$(CARGO) $@ $(CARGO_FLAGS)