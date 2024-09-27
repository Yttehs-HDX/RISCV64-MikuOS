.PHONY : all build run debug connect clean

ARCH := riscv64gc-unknown-none-elf
BUILD_TYPE := release
BIOS := opensbi.bin
KERNEL := kernel
TARGET := target/$(ARCH)/$(BUILD_TYPE)/$(KERNEL)
ADDR := 0x80200000

CARGO := cargo
CARGO_FLAGS := --$(BUILD_TYPE)

QEMU := qemu-system-riscv64
QEMU_FLAGS := -machine virt \
	-bios ../bootloader/$(BIOS) \
	-device loader,file=$(TARGET),addr=$(ADDR) \
	-nographic

GDB := riscv64-linux-gnu-gdb
GDB_FLAGS := -ex "file $(TARGET)" \
	-ex "target remote :1234" \
	-ex "set arch riscv:rv64" \
	-ex "layout src"

all: run

build:
	@$(CARGO) build $(CARGO_FLAGS)

run: build
	@$(QEMU) $(QEMU_FLAGS)

debug:
	@$(QEMU) $(QEMU_FLAGS) -S -s

connect:
	@$(GDB) $(GDB_FLAGS)

clean:
	@$(CARGO) clean

%:
	@$(CARGO) $@ $(CARGO_FLAGS)