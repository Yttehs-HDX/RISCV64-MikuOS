.PHONY : all build run debug connect fmt clean

ARCH := riscv64gc-unknown-none-elf
BUILD_TYPE := release
BIOS := rustsbi.bin
KERNEL := kernel
TARGET := target/$(ARCH)/$(BUILD_TYPE)/$(KERNEL)
FS_IMG := ../user/target/${ARCH}/${BUILD_TYPE}/sdcard.img

CARGO := cargo
CARGO_FLAGS := --$(BUILD_TYPE)

QEMU := qemu-system-riscv64
QEMU_FLAGS := -machine virt \
	-bios ../bootloader/$(BIOS) \
	-kernel $(TARGET) \
	-drive file=$(FS_IMG),if=none,format=raw,id=x0 \
	-device virtio-blk-device,drive=x0,bus=virtio-mmio-bus.0 \
	-nographic

GDB := riscv64-elf-gdb
GDB_FLAGS := -ex "file $(TARGET)" \
	-ex "target remote :1234" \
	-ex "set arch riscv:rv64"

all: run

build:
	@$(CARGO) build $(CARGO_FLAGS)

run: build
	@$(QEMU) $(QEMU_FLAGS)

debug:
	@echo -e "\033[33m[+] Run 'make connect' in another terminal at the same dir\033[0m"
	@$(QEMU) $(QEMU_FLAGS) -S -s

connect:
	@$(GDB) $(GDB_FLAGS)

fmt:
	@$(CARGO) fmt

clean:
	@$(CARGO) clean

%:
	@$(CARGO) $@ $(CARGO_FLAGS)
